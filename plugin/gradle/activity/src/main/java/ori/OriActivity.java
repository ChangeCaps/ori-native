package ori;

import android.os.Bundle;
import android.view.View;
import android.view.ViewGroup;
import android.view.WindowManager;
import android.widget.TextView;
import android.widget.FrameLayout;
import android.util.DisplayMetrics;
import android.graphics.Color;
import android.graphics.Typeface;
import android.text.Spannable;
import android.text.SpannableString;
import android.text.style.TypefaceSpan;
import android.text.style.AbsoluteSizeSpan;
import android.text.style.ForegroundColorSpan;
import android.text.style.StrikethroughSpan;

import androidx.appcompat.app.AppCompatActivity;

import java.util.Map;
import java.util.HashMap;
import java.lang.Long;

public class OriActivity extends AppCompatActivity {
    static {
        System.loadLibrary("native");
    }

    DisplayMetrics metrics;
    FrameLayout root;

    Map<Long, View> views = new HashMap();

    @Override
    public void onCreate(Bundle savedInstantanceState) {
        super.onCreate(savedInstantanceState);

        metrics = new DisplayMetrics();
        getWindowManager().getDefaultDisplay().getMetrics(metrics);

        root = new FrameLayout(this);
        setContentView(root);

        main();
    }

    native void main();

    public void removeView(long id) {
        runOnUiThread(() -> {
            View view = views.remove(id);
            ViewGroup parent = (ViewGroup) view.getParent();

            if (parent != null) {
                parent.removeView(view);
            }
        });
    }

    /* ---------- UNITS ---------- */

    public int px(float logical) {
        return (int) Math.round(logical * (float) metrics.density);
    }

    public float lc(int px) {
        return px / (float) metrics.density;
    }

    /* ---------- WINDOW ---------- */

    public void windowSetContents(long contents) {
        root.removeAllViews();
        root.addView(views.get(contents));
    }

    public void windowSetContentSize(float width, float height) {
        FrameLayout.LayoutParams lp = new FrameLayout.LayoutParams(
                px(width), px(height));

        root.getChildAt(0).setLayoutParams(lp);
    }

    public int windowGetWidth() {
        return (int) Math.round(lc(metrics.widthPixels));
    }

    public int windowGetHeight() {
        return (int) Math.round(lc(metrics.heightPixels));
    }

    /* ---------- GROUP ---------- */

    public void createGroup(long id) {
        OriGroup view = new OriGroup(this);
        views.put(id, view);
    }

    public void groupInsert(long id, int index, long child) {
        OriGroup view = (OriGroup) views.get(id);
        view.addView(views.get(child), index);
    }

    public void groupRemove(long id, int index) {
        OriGroup view = (OriGroup) views.get(id);
        view.removeViewAt(index);
    }

    public void groupSetChildLayout(long id, int index,
            float x, float y,
            float width, float height) {
        OriGroup view = (OriGroup) views.get(id);
        View child = view.getChildAt(index);

        OriGroup.LayoutParams lp = new OriGroup.LayoutParams(
                px(width), px(height),
                px(x), px(y));

        child.setLayoutParams(lp);
    }

    public void groupSetBackgroundColor(long id, float r, float g, float b, float a) {
        OriGroup view = (OriGroup) views.get(id);

        int color = Color.argb(a, r, g, b);
        view.setBackgroundColor(color);
    }

    public void groupSetCornerRadii(long id, float tl, float tr, float br, float bl) {
        OriGroup view = (OriGroup) views.get(id);
        view.setCornerRadii(px(tl), px(tr), px(br), px(bl));
    }

    public void groupSetBorderWidth(long id, float t, float r, float b, float l) {
        OriGroup view = (OriGroup) views.get(id);
        view.setBorderWidth(px(t), px(r), px(b), px(l));
    }

    public void groupSetBorderColor(long id, float r, float g, float b, float a) {
        OriGroup view = (OriGroup) views.get(id);

        int color = Color.argb(a, r, g, b);
        view.setBorderColor(color);
    }

    public void groupSetOverflow(long id, boolean visible) {
        OriGroup view = (OriGroup) views.get(id);
        view.setOverflow(visible);
    }

    public void groupSetShadow(long id,
            float r, float g, float b, float a,
            float dx, float dy,
            float blur, float spread) {
    }

    /* ---------- TEXT ---------- */

    public void createText(long id) {
        TextView view = new TextView(this);
        views.put(id, view);
    }

    public void textSetText(long id, String text, int wrap) {
        TextView view = (TextView) views.get(id);
        view.setText(new SpannableString(text));
    }

    public void textSetSpan(long id,
            int start, int end,
            float size,
            String family,
            int weight,
            int stretch,
            boolean italic,
            boolean strikethrough,
            float r,
            float g,
            float b,
            float a) {
        TextView view = (TextView) views.get(id);
        SpannableString text = new SpannableString(view.getText());

        Typeface typeface = Typeface.create(
                Typeface.create(family, 0),
                weight,
                italic);

        text.setSpan(
                new TypefaceSpan(typeface),
                start, end,
                Spannable.SPAN_EXCLUSIVE_EXCLUSIVE);

        text.setSpan(
                new AbsoluteSizeSpan(px(size)),
                start, end,
                Spannable.SPAN_EXCLUSIVE_EXCLUSIVE);

        if (strikethrough) {
            text.setSpan(
                    new StrikethroughSpan(),
                    start, end,
                    Spannable.SPAN_EXCLUSIVE_EXCLUSIVE);

        }

        int color = Color.argb(a, r, g, b);
        text.setSpan(
                new ForegroundColorSpan(color),
                start, end,
                Spannable.SPAN_EXCLUSIVE_EXCLUSIVE);

        view.setText(text);
    }

    public float textMeasureWidth(long id, float maxWidth) {
        TextView view = (TextView) views.get(id);

        int widthSpec = View.MeasureSpec.makeMeasureSpec(
                px(maxWidth), View.MeasureSpec.AT_MOST);
        int heightSpec = View.MeasureSpec.makeMeasureSpec(
                0, View.MeasureSpec.UNSPECIFIED);

        view.measure(widthSpec, heightSpec);

        return lc(view.getMeasuredWidth());
    }

    public float textMeasureHeight(long id, float maxWidth) {
        TextView view = (TextView) views.get(id);

        int widthSpec = View.MeasureSpec.makeMeasureSpec(
                px(maxWidth), View.MeasureSpec.AT_MOST);
        int heightSpec = View.MeasureSpec.makeMeasureSpec(
                0, View.MeasureSpec.UNSPECIFIED);

        view.measure(widthSpec, heightSpec);

        return lc(view.getMeasuredHeight());
    }
}
