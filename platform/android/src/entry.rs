use std::{
    ffi,
    process::Termination,
    sync::{Arc, Mutex, mpsc},
    thread,
};

use jni::{EnvUnowned, Outcome, objects::JObject};

use crate::application::{Event, GLOBAL_STATE, GlobalState};

/// # Safety
/// - `jni_env` must be a valid pointer to a valid JNI environment.
/// - `activity` must be a valid pointer to a valid JNI object.
pub unsafe fn entry<E>(jni_env: *mut ffi::c_void, activity: *mut ffi::c_void, main: fn() -> E)
where
    E: Termination + 'static,
{
    if let Some(state) = GLOBAL_STATE.get() {
        let activity = state
            .jvm
            .attach_current_thread(|env| {
                let activity = unsafe { JObject::from_raw(env, activity.cast()) };
                env.new_global_ref(activity)
            })
            .expect("creating global ref should work");

        *state.activity.lock().expect("mutex not poisoned") = Arc::new(activity);
        let _ = state.sender.send(Event::Recreate);

        return;
    }

    std::panic::set_hook(Box::new(|info| {
        if let Ok(message) = ffi::CString::new(info.to_string()) {
            unsafe {
                ndk_sys::__android_log_print(
                    ndk_sys::android_LogPriority::ANDROID_LOG_FATAL.0 as i32,
                    c"rust".as_ptr(),
                    message.as_ptr(),
                );
            }
        }

        std::process::abort();
    }));

    let mut env = unsafe { EnvUnowned::from_raw(jni_env.cast()) };
    let Outcome::Ok(jvm) = env.with_env(|env| env.get_java_vm()).into_outcome() else {
        panic!("failed getting jvm from jni env");
    };

    let activity = jvm
        .attach_current_thread(|env| {
            let activity = unsafe { JObject::from_raw(env, activity.cast()) };
            env.new_global_ref(activity)
        })
        .expect("creating global ref should work");

    let (sender, receiver) = mpsc::channel();

    let state = GlobalState {
        sender,
        jvm,
        receiver: Mutex::new(receiver),
        activity: Mutex::new(Arc::new(activity)),
    };

    let _ = GLOBAL_STATE.set(state);

    thread::spawn(move || {
        let exit_code = main().report();

        if exit_code == std::process::ExitCode::SUCCESS {
            std::process::exit(0);
        } else {
            std::process::exit(1);
        }
    });
}
