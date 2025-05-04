package javax.swing;

import java.awt.event.ActionListener;
import java.util.ArrayList;

public class Timer {
    private static ArrayList<Timer> allTimers = new ArrayList<Timer>();

    private int delay;

    // ActionListeners are passed into a closure handed off to JS; let's make sure
    // they don't get collected by shoving them in here.
    private ActionListener currentListener;

    public Timer(int delay, ActionListener listener) {
        this.delay = delay;

        if (listener != null) {
            this.addActionListener(listener);
        }

        Timer.allTimers.add(this);
    }

    public void addActionListener(ActionListener listener) {
        // FIXME only supports one listener at a time
        if (this.currentListener == null) {
            this.currentListener = listener;
        }
    }

    public void start() {
        Timer.internalStartTimer(this.delay, this.currentListener);
    }

    private static native void internalStartTimer(int delay, ActionListener listener);
}
