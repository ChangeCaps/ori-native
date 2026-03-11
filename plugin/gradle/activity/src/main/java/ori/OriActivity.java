package ori;

import android.app.Activity;
import android.os.Bundle;
import android.widget.TextView;

public class OriActivity extends Activity {
    @Override
    public void onCreate(Bundle savedInstantanceState) {
        super.onCreate(savedInstantanceState);
        TextView view = new TextView(this);
        view.setText("Hello android");
        setContentView(view);
    }

    native void main();
}
