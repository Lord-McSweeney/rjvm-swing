package javax.swing;

import java.awt.event.ActionListener;

public class Timer {
    private int delay;

    public Timer(int delay, ActionListener listener) {
        this.delay = delay;

        if (listener != null) {
            this.addActionListener(listener);
        }
    }

    public void addActionListener(ActionListener listener) {
        // TODO implement
    }

    public void start() {
        // TODO implement
    }
}
