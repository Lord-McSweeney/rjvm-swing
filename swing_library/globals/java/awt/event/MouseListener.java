package java.awt.event;

import java.util.EventListener;

public interface MouseListener extends EventListener {
    void mouseClicked(MouseEvent e);

    void mousePressed(MouseEvent e);

    void mouseReleased(MouseEvent e);

    void mouseEntered(MouseEvent e);

    void mouseExited(MouseEvent e);
}
