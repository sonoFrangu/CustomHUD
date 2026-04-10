package org.frangu.customhud.client;

import net.minecraft.client.gui.DrawContext;
import net.minecraft.client.gui.screen.Screen;
import net.minecraft.client.gui.widget.ButtonWidget;
import net.minecraft.text.Text;

public class ConfigScreen extends Screen {
    private final Screen parent;

    public ConfigScreen(Screen parent) {
        super(Text.literal("Beautiful Day Counter - Settings"));
        this.parent = parent;
    }

    @Override
    protected void init() {
        int centerX = this.width / 2;
        int centerY = this.height / 2;

        // RIGA 1: Orologio 12/24h  |  Posizione Sopra/Sotto
        this.addDrawableChild(ButtonWidget.builder(getFormatText(), button -> {
            CustomHudConfig.INSTANCE.use12HourFormat = !CustomHudConfig.INSTANCE.use12HourFormat;
            CustomHudConfig.save();
            button.setMessage(getFormatText());
        }).dimensions(centerX - 105, centerY - 45, 100, 20).build());

        this.addDrawableChild(ButtonWidget.builder(getPositionText(), button -> {
            CustomHudConfig.INSTANCE.positionBottom = !CustomHudConfig.INSTANCE.positionBottom;
            CustomHudConfig.save();
            button.setMessage(getPositionText());
        }).dimensions(centerX + 5, centerY - 45, 100, 20).build());

        // RIGA 2: Mostra Giorni  |  Mostra Orologio
        this.addDrawableChild(ButtonWidget.builder(getDayText(), button -> {
            CustomHudConfig.INSTANCE.showDayCounter = !CustomHudConfig.INSTANCE.showDayCounter;
            CustomHudConfig.save();
            button.setMessage(getDayText());
        }).dimensions(centerX - 105, centerY - 20, 100, 20).build());

        this.addDrawableChild(ButtonWidget.builder(getClockText(), button -> {
            CustomHudConfig.INSTANCE.showClock = !CustomHudConfig.INSTANCE.showClock;
            CustomHudConfig.save();
            button.setMessage(getClockText());
        }).dimensions(centerX + 5, centerY - 20, 100, 20).build());

        // RIGA 3: Mostra Bossbar (Centrato)
        this.addDrawableChild(ButtonWidget.builder(getBossbarText(), button -> {
            CustomHudConfig.INSTANCE.showBossbar = !CustomHudConfig.INSTANCE.showBossbar;
            CustomHudConfig.save();
            button.setMessage(getBossbarText());
        }).dimensions(centerX - 50, centerY + 5, 100, 20).build());

        // PULSANTE FATTO
        this.addDrawableChild(ButtonWidget.builder(Text.literal("Done"), button -> {
            this.client.setScreen(this.parent);
        }).dimensions(centerX - 100, centerY + 40, 200, 20).build());
    }

    // Metodi per aggiornare il testo dei pulsanti dinamicamente
    private Text getFormatText() { return Text.literal("Format: " + (CustomHudConfig.INSTANCE.use12HourFormat ? "12h" : "24h")); }
    private Text getPositionText() { return Text.literal("Pos: " + (CustomHudConfig.INSTANCE.positionBottom ? "Bottom" : "Top")); }
    private Text getDayText() { return Text.literal("Days: " + (CustomHudConfig.INSTANCE.showDayCounter ? "ON" : "OFF")); }
    private Text getClockText() { return Text.literal("Clock: " + (CustomHudConfig.INSTANCE.showClock ? "ON" : "OFF")); }
    private Text getBossbarText() { return Text.literal("Bossbar: " + (CustomHudConfig.INSTANCE.showBossbar ? "ON" : "OFF")); }

    @Override
    public void render(DrawContext context, int mouseX, int mouseY, float delta) {
        super.render(context, mouseX, mouseY, delta);
        context.drawCenteredTextWithShadow(this.textRenderer, this.title, this.width / 2, 20, 0xFFFFFF);
    }

    @Override
    public void close() {
        this.client.setScreen(this.parent);
    }
}