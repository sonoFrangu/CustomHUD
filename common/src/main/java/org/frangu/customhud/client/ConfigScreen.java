package org.frangu.customhud.client;

import net.minecraft.client.gui.GuiGraphicsExtractor;
import net.minecraft.client.gui.components.Button;
import net.minecraft.client.gui.screens.Screen;
import net.minecraft.network.chat.Component;

public final class ConfigScreen extends Screen {
    private final Screen parent;

    public ConfigScreen(Screen parent) {
        super(Component.translatable("screen.customhud.title"));
        this.parent = parent;
    }

    @Override
    protected void init() {
        int centerX = this.width / 2;
        int centerY = this.height / 2;

        this.addRenderableWidget(Button.builder(formatText(), button -> {
            CustomHudConfig.INSTANCE.use12HourFormat = !CustomHudConfig.INSTANCE.use12HourFormat;
            CustomHudConfig.save();
            button.setMessage(formatText());
        }).bounds(centerX - 105, centerY - 45, 100, 20).build());

        this.addRenderableWidget(Button.builder(positionText(), button -> {
            CustomHudConfig.INSTANCE.positionBottom = !CustomHudConfig.INSTANCE.positionBottom;
            CustomHudConfig.save();
            button.setMessage(positionText());
        }).bounds(centerX + 5, centerY - 45, 100, 20).build());

        this.addRenderableWidget(Button.builder(dayText(), button -> {
            CustomHudConfig.INSTANCE.showDayCounter = !CustomHudConfig.INSTANCE.showDayCounter;
            CustomHudConfig.save();
            button.setMessage(dayText());
        }).bounds(centerX - 105, centerY - 20, 100, 20).build());

        this.addRenderableWidget(Button.builder(clockText(), button -> {
            CustomHudConfig.INSTANCE.showClock = !CustomHudConfig.INSTANCE.showClock;
            CustomHudConfig.save();
            button.setMessage(clockText());
        }).bounds(centerX + 5, centerY - 20, 100, 20).build());

        this.addRenderableWidget(Button.builder(bossbarText(), button -> {
            CustomHudConfig.INSTANCE.showBossbar = !CustomHudConfig.INSTANCE.showBossbar;
            CustomHudConfig.save();
            button.setMessage(bossbarText());
        }).bounds(centerX - 50, centerY + 5, 100, 20).build());

        this.addRenderableWidget(Button.builder(Component.translatable("gui.done"), button -> this.onClose())
                .bounds(centerX - 100, centerY + 40, 200, 20)
                .build());
    }

    @Override
    public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float partialTick) {
        this.extractBackground(graphics, mouseX, mouseY, partialTick);
        super.extractRenderState(graphics, mouseX, mouseY, partialTick);
        graphics.centeredText(this.font, this.title, this.width / 2, 20, 0xFFFFFFFF);
    }

    @Override
    public void onClose() {
        this.minecraft.setScreen(this.parent);
    }

    private static Component formatText() {
        return Component.translatable("screen.customhud.format", CustomHudConfig.INSTANCE.use12HourFormat ? "12h" : "24h");
    }

    private static Component positionText() {
        return Component.translatable("screen.customhud.position", Component.translatable(
                CustomHudConfig.INSTANCE.positionBottom ? "screen.customhud.bottom" : "screen.customhud.top"));
    }

    private static Component dayText() {
        return Component.translatable("screen.customhud.days",
                Component.translatable(CustomHudConfig.INSTANCE.showDayCounter ? "screen.customhud.on" : "screen.customhud.off"));
    }

    private static Component clockText() {
        return Component.translatable("screen.customhud.clock",
                Component.translatable(CustomHudConfig.INSTANCE.showClock ? "screen.customhud.on" : "screen.customhud.off"));
    }

    private static Component bossbarText() {
        return Component.translatable("screen.customhud.bossbar",
                Component.translatable(CustomHudConfig.INSTANCE.showBossbar ? "screen.customhud.on" : "screen.customhud.off"));
    }
}
