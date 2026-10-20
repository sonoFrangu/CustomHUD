package org.frangu.customhud.client;

import net.minecraft.client.gui.components.Button;
import net.minecraft.client.gui.components.CycleButton;
import net.minecraft.client.gui.components.Tooltip;
import net.minecraft.client.gui.layouts.GridLayout;
import net.minecraft.client.gui.layouts.HeaderAndFooterLayout;
import net.minecraft.client.gui.screens.Screen;
import net.minecraft.network.chat.CommonComponents;
import net.minecraft.network.chat.Component;

import java.util.function.BiConsumer;

public class ConfigScreen extends Screen {
    private static final int BUTTON_WIDTH = 150;

    private final Screen parent;
    private final HeaderAndFooterLayout layout = new HeaderAndFooterLayout(this);

    public ConfigScreen(Screen parent) {
        super(Component.translatable("customhud.config.title"));
        this.parent = parent;
    }

    @Override
    protected void init() {
        CustomHudConfig config = CustomHudConfig.INSTANCE;
        layout.addTitleHeader(this.title, this.font);

        GridLayout grid = new GridLayout().columnSpacing(10).rowSpacing(4);
        GridLayout.RowHelper rows = grid.createRowHelper(2);

        rows.addChild(choice("format", "customhud.config.format.12h", "customhud.config.format.24h",
                config.use12HourFormat, (c, v) -> c.use12HourFormat = v));
        rows.addChild(choice("position", "customhud.config.position.bottom", "customhud.config.position.top",
                config.positionBottom, (c, v) -> c.positionBottom = v));
        rows.addChild(toggle("days", config.showDayCounter, (c, v) -> c.showDayCounter = v));
        rows.addChild(toggle("clock", config.showClock, (c, v) -> c.showClock = v));
        rows.addChild(toggle("bossbar", config.showBossbar, (c, v) -> c.showBossbar = v), 2,
                rows.newCellSettings().alignHorizontallyCenter());

        layout.addToContents(grid);
        layout.addToFooter(Button.builder(CommonComponents.GUI_DONE, button -> onClose()).width(200).build());

        layout.visitWidgets(this::addRenderableWidget);
        repositionElements();
    }

    private CycleButton<Boolean> toggle(String key, boolean value, BiConsumer<CustomHudConfig, Boolean> setter) {
        return CycleButton.onOffBuilder(value)
                .withTooltip(v -> Tooltip.create(Component.translatable("customhud.config." + key + ".tooltip")))
                .create(0, 0, BUTTON_WIDTH, 20, Component.translatable("customhud.config." + key), (b, v) -> apply(setter, v));
    }

    private CycleButton<Boolean> choice(String key, String trueKey, String falseKey, boolean value,
                                        BiConsumer<CustomHudConfig, Boolean> setter) {
        return CycleButton.booleanBuilder(Component.translatable(trueKey), Component.translatable(falseKey), value)
                .withTooltip(v -> Tooltip.create(Component.translatable("customhud.config." + key + ".tooltip")))
                .create(0, 0, BUTTON_WIDTH, 20, Component.translatable("customhud.config." + key), (b, v) -> apply(setter, v));
    }

    private static void apply(BiConsumer<CustomHudConfig, Boolean> setter, boolean value) {
        setter.accept(CustomHudConfig.INSTANCE, value);
        CustomHudConfig.save();
    }

    @Override
    protected void repositionElements() {
        layout.arrangeElements();
    }

    @Override
    public void onClose() {
        this.minecraft.gui.setScreen(this.parent);
    }
}
