package javax.swing;

import java.awt.Frame;

public class JFrame extends Frame {
    public static final int DO_NOTHING_ON_CLOSE = 0;
    public static final int EXIT_ON_CLOSE = 3;

    public JFrame(String name) {
        super(name);
    }

    public void setDefaultCloseOperation(int operation) { }
}
