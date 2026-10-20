package org.frangu.customhud.client;

import com.mojang.blaze3d.platform.InputConstants;
import net.minecraft.client.DeltaTracker;
import net.minecraft.client.KeyMapping;
import net.minecraft.client.Minecraft;
import net.minecraft.client.gui.GuiGraphicsExtractor;
import net.minecraft.resources.Identifier;
import org.frangu.customhud.Customhud;
import org.joml.Matrix3x2fStack;

import java.nio.file.Path;

/** Logica client condivisa: ogni loader chiama questi metodi dai propri eventi. */
public final class CustomhudClient {
    public static final Identifier HUD_LAYER_ID = Identifier.fromNamespaceAndPath(Customhud.MOD_ID, "day_hud");

    public static final KeyMapping CONFIG_KEY = new KeyMapping(
            "key.customhud.open_config",
            InputConstants.Type.KEYSYM,
            InputConstants.KEY_H,
            KeyMapping.Category.MISC
    );

    private static final DayHud HUD = new DayHud();

    private CustomhudClient() {
    }

    public static void init(Path configDirectory) {
        CustomHudConfig.load(configDirectory);
    }

    public static void onEndClientTick(Minecraft client) {
        while (CONFIG_KEY.consumeClick()) {
            client.setScreen(new ConfigScreen(client.screen));
        }
    }

    public static void extractHud(GuiGraphicsExtractor graphics, DeltaTracker deltaTracker) {
        HUD.extract(graphics, deltaTracker);
    }

    /** Le bossbar vanilla vanno spostate sotto l'HUD quando è visibile in alto. */
    public static boolean shouldShiftBossBars() {
        CustomHudConfig config = CustomHudConfig.INSTANCE;
        return config.isHudVisible() && !config.positionBottom;
    }

    public static void pushBossBarShift(GuiGraphicsExtractor graphics) {
        Matrix3x2fStack pose = graphics.pose();
        pose.pushMatrix();
        pose.translate(0.0f, DayHud.bossBarShift());
    }

    public static void popBossBarShift(GuiGraphicsExtractor graphics) {
        graphics.pose().popMatrix();
    }
}
