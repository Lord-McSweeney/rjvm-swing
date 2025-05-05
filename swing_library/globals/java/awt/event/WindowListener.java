package java.awt.event;

public interface WindowListener {
    void windowActivated(WindowEvent e);
    void windowClosed(WindowEvent e);
    void windowClosing(WindowEvent e);
    void windowDeactivated(WindowEvent e);
    void windowDeiconified(WindowEvent e);
    void windowIconified(WindowEvent e);
    void windowOpened(WindowEvent e);
}
