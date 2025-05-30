package java.awt;

public abstract class Graphics2D extends Graphics { }

class CRC2DGraphics extends Graphics2D {
    private Font font;

    CRC2DGraphics() {
        super();
    }

    public native void drawLine(int x1, int y1, int x2, int y2);

    public void drawString(String str, int x, int y) {
        // TODO native implementation
    }

    public native void fillRect(int x, int y, int width, int height);

    public Font getFont() {
        return this.font;
    }

    public FontMetrics getFontMetrics(Font font) {
        // TODO implement
        return new FontMetrics(null);
    }

    public native void setColor(Color color);

    public void setFont(Font font) {
        this.font = font;
    }

    public native void translate(int x, int y);
}
