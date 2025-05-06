package javax.swing;

import java.awt.Container;
import java.awt.Graphics;

public abstract class JComponent extends Container {
    public JComponent() { }

    // TODO this should be `protected` but our current design calls
    // it from `Component.repaint`
    public void paintComponent(Graphics g) {
        // do we need to implement anything here?
    }
}
