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
        if (JPanel.mainPanel instanceof MouseMotionListener) {
            MouseEvent evt = new MouseEvent(JPanel.mainPanel, 0, 0, 0, x, y, 0, 0, 0, false, 0);
            MouseMotionListener listener = (MouseMotionListener) JPanel.mainPanel;
            listener.mouseMoved(evt);
        }
    }
    private static void globalMouseDown(int x, int y) {
        if (JPanel.mainPanel instanceof MouseListener) {
            MouseEvent evt = new MouseEvent(JPanel.mainPanel, 0, 0, 0, x, y, 0, 0, 0, false, 0);
            MouseListener listener = (MouseListener) JPanel.mainPanel;
            listener.mousePressed(evt);
        }
    }
    private static void globalMouseUp(int x, int y) {
        if (JPanel.mainPanel instanceof MouseListener) {
            MouseEvent evt = new MouseEvent(JPanel.mainPanel, 0, 0, 0, x, y, 0, 0, 0, false, 0);
            MouseListener listener = (MouseListener) JPanel.mainPanel;
            listener.mouseReleased(evt);
        }
    }
}
