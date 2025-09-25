package java.awt;

import java.awt.event.KeyListener;
import java.awt.event.MouseListener;
import java.awt.event.MouseMotionListener;

import javax.swing.JComponent;

public abstract class Component {
    protected Component() { }

    public void addKeyListener(KeyListener listener) {
        // TODO implement
    }

    public void addMouseListener(MouseListener listener) {
        // TODO implement
    }

    public void addMouseMotionListener(MouseMotionListener listener) {
        // TODO implement
    }

    public void setCursor(Cursor cursor) {
        // TODO implement
    }

    public void setFocusable(boolean focusable) {
        // TODO implement
    }

    public void setVisible(boolean visible) {
        // TODO implement
    }

    public void repaint() {
        // TODO implement properly

        // FIXME please
        this.startPaint();
        if (this instanceof JComponent) {
            JComponent self = (JComponent) this;
            self.paintComponent(new CRC2DGraphics());
        }
        this.flushPaint();
    }

    private native void startPaint();
    private native void flushPaint();
}
