package org.frangu.customhud.client;

import net.minecraft.client.gui.components.AbstractSliderButton;
import net.minecraft.client.gui.components.Button;
import net.minecraft.client.gui.components.CycleButton;
import net.minecraft.client.gui.components.StringWidget;
import net.minecraft.client.gui.components.Tooltip;
import net.minecraft.client.gui.layouts.GridLayout;
import net.minecraft.client.gui.layouts.HeaderAndFooterLayout;
import net.minecraft.client.gui.layouts.LinearLayout;
import net.minecraft.client.gui.screens.Screen;
import net.minecraft.network.chat.CommonComponents;
import net.minecraft.network.chat.Component;

import java.util.Locale;
import java.util.function.BiConsumer;
import java.util.function.IntConsumer;

public class ConfigScreen extends Screen {
    private static final int BUTTON_WIDTH = 150;
    private static final int SECTION_BUTTON_WIDTH = 100;

    private enum Section { ELEMENTS, LOOK, BEHAVIOR }

    /** Ricorda l'ultima sezione aperta finché il gioco resta acceso. */
    private static Section section = Section.ELEMENTS;

    private final Screen parent;
    private HeaderAndFooterLayout layout;

    public ConfigScreen(Screen parent) {
        super(Component.translatable("customhud.config.title"));
        this.parent = parent;
    }

    @Override
    protected void init() {
        CustomHudConfig config = CustomHudConfig.INSTANCE;
        layout = new HeaderAndFooterLayout(this, 58, 33);

        LinearLayout header = layout.addToHeader(LinearLayout.vertical().spacing(8));
        header.defaultCellSetting().alignHorizontallyCenter();
        header.addChild(new StringWidget(this.title, this.font));
        LinearLayout sections = header.addChild(LinearLayout.horizontal().spacing(4));
        for (Section s : Section.values()) {
            Button button = Button.builder(Component.translatable("customhud.config.section." + key(s)), b -> {
                section = s;
                rebuildWidgets();
            }).width(SECTION_BUTTON_WIDTH).build();
            button.active = s != section;
            sections.addChild(button);
        }

        GridLayout grid = new GridLayout().columnSpacing(10).rowSpacing(4);
        GridLayout.RowHelper rows = grid.createRowHelper(2);
        switch (section) {
            case ELEMENTS -> {
                rows.addChild(toggle("days", config.showDayCounter, (c, v) -> c.showDayCounter = v));
                rows.addChild(toggle("clock", config.showClock, (c, v) -> c.showClock = v));
                rows.addChild(toggle("bossbar", config.showBossbar, (c, v) -> c.showBossbar = v));
                rows.addChild(toggle("night_ticks", config.showNightTicks, (c, v) -> c.showNightTicks = v));
                rows.addChild(toggle("sleep_icon", config.showSleepIcon, (c, v) -> c.showSleepIcon = v));
                rows.addChild(toggle("moon_phase", config.showMoonPhase, (c, v) -> c.showMoonPhase = v));
                rows.addChild(toggle("countdown", config.showNightCountdown, (c, v) -> c.showNightCountdown = v), 2,
                        rows.newCellSettings().alignHorizontallyCenter());
            }
            case LOOK -> {
                rows.addChild(new IntSlider("scale", CustomHudConfig.MIN_SCALE, CustomHudConfig.MAX_SCALE, 5, "%",
                        config.scale, v -> CustomHudConfig.INSTANCE.scale = v));
                rows.addChild(new IntSlider("opacity", CustomHudConfig.MIN_OPACITY, CustomHudConfig.MAX_OPACITY, 5, "%",
                        config.opacity, v -> CustomHudConfig.INSTANCE.opacity = v));
                rows.addChild(toggle("background", config.showBackground, (c, v) -> c.showBackground = v));
                rows.addChild(choice("theme", CustomHudConfig.BarTheme.values(), config.barTheme, (c, v) -> c.barTheme = v));
            }
            case BEHAVIOR -> {
                rows.addChild(choice("format", "customhud.config.format.12h", "customhud.config.format.24h",
                        config.use12HourFormat, (c, v) -> c.use12HourFormat = v));
                rows.addChild(choice("position", "customhud.config.position.bottom", "customhud.config.position.top",
                        config.positionBottom, (c, v) -> c.positionBottom = v));
                rows.addChild(new IntSlider("offset_x", -CustomHudConfig.MAX_OFFSET, CustomHudConfig.MAX_OFFSET, 1, "",
                        config.offsetX, v -> CustomHudConfig.INSTANCE.offsetX = v));
                rows.addChild(new IntSlider("offset_y", -CustomHudConfig.MAX_OFFSET, CustomHudConfig.MAX_OFFSET, 1, "",
                        config.offsetY, v -> CustomHudConfig.INSTANCE.offsetY = v));
                rows.addChild(choice("dimensions", CustomHudConfig.OtherDimensions.values(), config.otherDimensions,
                        (c, v) -> c.otherDimensions = v));
                rows.addChild(toggle("milestones", config.milestones, (c, v) -> c.milestones = v));
                rows.addChild(toggle("milestone_sound", config.milestoneSound, (c, v) -> c.milestoneSound = v), 2,
                        rows.newCellSettings().alignHorizontallyCenter());
            }
        }
        layout.addToContents(grid);

        LinearLayout footer = layout.addToFooter(LinearLayout.horizontal().spacing(8));
        footer.addChild(Button.builder(Component.translatable("customhud.config.reset"), button -> {
            CustomHudConfig.reset();
            rebuildWidgets();
        }).width(BUTTON_WIDTH).build());
        footer.addChild(Button.builder(CommonComponents.GUI_DONE, button -> onClose()).width(BUTTON_WIDTH).build());

        layout.visitWidgets(this::addRenderableWidget);
        repositionElements();
    }

    private static String key(Enum<?> value) {
        return value.name().toLowerCase(Locale.ROOT);
    }

    private static Tooltip tooltip(String key) {
        return Tooltip.create(Component.translatable("customhud.config." + key + ".tooltip"));
    }

    private CycleButton<Boolean> toggle(String key, boolean value, BiConsumer<CustomHudConfig, Boolean> setter) {
        return CycleButton.onOffBuilder(value)
                .withTooltip(v -> tooltip(key))
                .create(0, 0, BUTTON_WIDTH, 20, Component.translatable("customhud.config." + key), (b, v) -> apply(setter, v));
    }

    private CycleButton<Boolean> choice(String key, String trueKey, String falseKey, boolean value,
                                        BiConsumer<CustomHudConfig, Boolean> setter) {
        return CycleButton.booleanBuilder(Component.translatable(trueKey), Component.translatable(falseKey), value)
                .withTooltip(v -> tooltip(key))
                .create(0, 0, BUTTON_WIDTH, 20, Component.translatable("customhud.config." + key), (b, v) -> apply(setter, v));
    }

    private <T extends Enum<T>> CycleButton<T> choice(String key, T[] values, T value, BiConsumer<CustomHudConfig, T> setter) {
        return CycleButton.builder((T v) -> Component.translatable("customhud.config." + key + "." + key(v)), value)
                .withValues(values)
                .withTooltip(v -> tooltip(key))
                .create(0, 0, BUTTON_WIDTH, 20, Component.translatable("customhud.config." + key), (b, v) -> apply(setter, v));
    }

    private static <T> void apply(BiConsumer<CustomHudConfig, T> setter, T value) {
        setter.accept(CustomHudConfig.INSTANCE, value);
        CustomHudConfig.save();
    }

    @Override
    protected void repositionElements() {
        layout.arrangeElements();
    }

    @Override
    public void removed() {
        // Gli slider cambiano il valore a ogni movimento: si salva una volta sola all'uscita
        CustomHudConfig.save();
    }

    @Override
    public void onClose() {
        this.minecraft.setScreen(this.parent);
    }

    /** Slider con valori interi a passi fissi, es. "Dimensione: 110%". */
    private static final class IntSlider extends AbstractSliderButton {
        private final String key;
        private final int min, max, step;
        private final String suffix;
        private final IntConsumer setter;

        IntSlider(String key, int min, int max, int step, String suffix, int value, IntConsumer setter) {
            super(0, 0, BUTTON_WIDTH, 20, Component.empty(), (value - min) / (double) (max - min));
            this.key = key;
            this.min = min;
            this.max = max;
            this.step = step;
            this.suffix = suffix;
            this.setter = setter;
            setTooltip(tooltip(key));
            updateMessage();
        }

        private int current() {
            return min + (int) Math.round(value * (max - min) / step) * step;
        }

        @Override
        protected void updateMessage() {
            setMessage(CommonComponents.optionNameValue(Component.translatable("customhud.config." + key),
                    Component.literal(current() + suffix)));
        }

        @Override
        protected void applyValue() {
            setter.accept(current());
        }
    }
}
