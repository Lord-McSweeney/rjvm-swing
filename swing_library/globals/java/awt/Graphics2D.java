package java.awt;

public abstract class Graphics2D extends Graphics { }

class CRC2DGraphics extends Graphics2D {
    private Font font;

    CRC2DGraphics() {
        super();
    }

    public void drawLine(int x1, int y1, int x2, int y2) {
        // TODO native implementation
    }

    public void drawString(String str, int x, int y) {
        // TODO native implementation
    }

    public void fillRect(int x, int y, int width, int height) {
        // TODO native implementation
    }

    public Font getFont() {
        return this.font;
    }

    public FontMetrics getFontMetrics(Font font) {
        // TODO implement
        return new FontMetrics(null);
    }

    public void setColor(Color color) {
        // TODO native implementation
    }

    public void setFont(Font font) {
        this.font = font;
    }

    public void translate(int x, int y) {
        // TODO native implementation
    }
}
