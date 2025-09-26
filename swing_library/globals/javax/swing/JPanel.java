package javax.swing;

import java.awt.event.MouseEvent;
import java.awt.event.MouseListener;
import java.awt.event.MouseMotionListener;

public class JPanel extends JComponent {
    private static JPanel mainPanel;

    public JPanel() {
        JPanel.mainPanel = this;
    }

    // Accessed from native code
    private static void globalMouseMoved(int x, int y) {
        MouseEvent evt = new MouseEvent(null, 0, 0, 0, x, y, 0, 0, 0, false, 0);
        if (JPanel.mainPanel instanceof MouseMotionListener) {
            MouseMotionListener listener = (MouseMotionListener) JPanel.mainPanel;
            listener.mouseMoved(evt);
        }
    }
    private static void globalMouseDown(int x, int y) {
        MouseEvent evt = new MouseEvent(null, 0, 0, 0, x, y, 0, 0, 0, false, 0);
        if (JPanel.mainPanel instanceof MouseListener) {
            MouseListener listener = (MouseListener) JPanel.mainPanel;
            listener.mousePressed(evt);
        }
    }
    private static void globalMouseUp(int x, int y) {
        MouseEvent evt = new MouseEvent(null, 0, 0, 0, x, y, 0, 0, 0, false, 0);
        if (JPanel.mainPanel instanceof MouseListener) {
            MouseListener listener = (MouseListener) JPanel.mainPanel;
            listener.mouseReleased(evt);
        }
    }
}
