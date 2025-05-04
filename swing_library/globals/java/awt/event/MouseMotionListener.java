package java.awt.event;

import java.util.EventListener;

public interface MouseMotionListener extends EventListener {
    void mouseDragged(MouseEvent e);

    void mouseMoved(MouseEvent e);
}
