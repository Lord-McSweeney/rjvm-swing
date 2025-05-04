package java.awt.event;

import java.util.EventListener;

public interface KeyListener extends EventListener {
    void keyTyped(KeyEvent e);

    void keyPressed(KeyEvent e);

    void keyReleased(KeyEvent e);
}
