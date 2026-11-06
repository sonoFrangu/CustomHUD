package org.frangu.customhud.client;

import com.mojang.blaze3d.platform.InputConstants;
import net.minecraft.client.gui.GuiGraphics;
import net.minecraft.client.gui.components.Button;
import net.minecraft.client.gui.screens.Screen;
import net.minecraft.client.input.KeyEvent;
import net.minecraft.client.input.MouseButtonEvent;
import net.minecraft.network.chat.CommonComponents;
import net.minecraft.network.chat.Component;
import net.minecraft.util.FormattedCharSequence;

import java.util.List;

/**
 * Spostamento col mouse del contatore semplice: il mondo resta visibile con sopra il
 * contatore vero (lo disegna l'HUD), un bordo che mostra dove afferrarlo, una riga di
 * aiuto e Fatto. Le frecce lo spostano di un punto percentuale, Esc torna indietro.
 */
public class PlaceScreen extends Screen {
    private static final int BUTTON_WIDTH = 150;
    private static final int FOOTER_HEIGHT = 33;

    private final Screen parent;
    private boolean grabbed;
    // Dove lo si è afferrato rispetto al suo angolo
    private double grabX, grabY;

    public PlaceScreen(Screen parent) {
        super(Component.translatable("customhud.config.place"));
        this.parent = parent;
    }

    @Override
    protected void init() {
        addRenderableWidget(Button.builder(CommonComponents.GUI_DONE, button -> onClose())
                .bounds((this.width - BUTTON_WIDTH) / 2, this.height - FOOTER_HEIGHT + (FOOTER_HEIGHT - 20) / 2,
                        BUTTON_WIDTH, 20)
                .build());
    }

    /** Il riquadro del contatore com'è adesso; null fuori da un mondo. */
    private DayHud.SimpleBox box() {
        if (this.minecraft.level == null) return null;
        int textWidth = this.font.width(DayHud.currentDayText(this.minecraft.level));
        return DayHud.simpleBox(CustomHudConfig.INSTANCE, this.width, this.height, textWidth);
    }

    @Override
    public void renderBackground(GuiGraphics graphics, int mouseX, int mouseY, float a) {
        // Niente sfocatura né fondo: si deve vedere dove finisce il contatore sul mondo
    }

    @Override
    public void render(GuiGraphics graphics, int mouseX, int mouseY, float a) {
        List<FormattedCharSequence> lines = this.font.split(Component.translatable("customhud.place.hint"),
                Math.min(this.width - 40, 300));
        int top = (this.height - lines.size() * 10) / 2;
        for (int i = 0; i < lines.size(); i++) {
            graphics.drawCenteredString(this.font, lines.get(i), this.width / 2, top + i * 10, 0xFFAAAAAA);
        }

        DayHud.SimpleBox box = box();
        if (box != null) {
            // Bordo di 1 pixel attorno al riquadro: bianco se afferrato o sotto il mouse
            boolean lit = grabbed || box.contains(mouseX, mouseY);
            int w = Math.round(box.width() * box.scale()) + 2;
            int h = Math.round(box.height() * box.scale()) + 2;
            graphics.renderOutline(box.x() - 1, box.y() - 1, w, h, lit ? 0xFFFFFFFF : 0x80FFFFFF);
        }
        super.render(graphics, mouseX, mouseY, a);
    }

    @Override
    public boolean mouseClicked(MouseButtonEvent event, boolean doubleClick) {
        DayHud.SimpleBox box = box();
        if (event.button() == InputConstants.MOUSE_BUTTON_LEFT && box != null && box.contains(event.x(), event.y())) {
            grabbed = true;
            grabX = event.x() - box.x();
            grabY = event.y() - box.y();
            return true;
        }
        return super.mouseClicked(event, doubleClick);
    }

    @Override
    public boolean mouseDragged(MouseButtonEvent event, double dx, double dy) {
        DayHud.SimpleBox box = box();
        if (!grabbed || box == null) return super.mouseDragged(event, dx, dy);
        // L'angolo segue il cursore e diventa la percentuale dello spazio libero
        CustomHudConfig config = CustomHudConfig.INSTANCE;
        config.simpleX = percent(event.x() - grabX, this.width, box.width(), box.scale());
        config.simpleY = percent(event.y() - grabY, this.height, box.height(), box.scale());
        return true;
    }

    private static int percent(double position, int screen, int size, float scale) {
        float free = DayHud.freeSpace(screen, size, scale);
        if (free <= 0) return 0;
        return (int) Math.clamp(Math.round(position / free * 100), 0, 100);
    }

    @Override
    public boolean mouseReleased(MouseButtonEvent event) {
        if (grabbed && event.button() == InputConstants.MOUSE_BUTTON_LEFT) {
            grabbed = false;
            CustomHudConfig.save();
            return true;
        }
        return super.mouseReleased(event);
    }

    @Override
    public boolean keyPressed(KeyEvent event) {
        int dx = 0, dy = 0;
        switch (event.key()) {
            case InputConstants.KEY_LEFT -> dx = -1;
            case InputConstants.KEY_RIGHT -> dx = 1;
            case InputConstants.KEY_UP -> dy = -1;
            case InputConstants.KEY_DOWN -> dy = 1;
            default -> {
                return super.keyPressed(event);
            }
        }
        CustomHudConfig config = CustomHudConfig.INSTANCE;
        config.simpleX = Math.clamp(config.simpleX + dx, 0, 100);
        config.simpleY = Math.clamp(config.simpleY + dy, 0, 100);
        CustomHudConfig.save();
        return true;
    }

    @Override
    public void onClose() {
        CustomHudConfig.save();
        this.minecraft.setScreen(this.parent);
    }
}
