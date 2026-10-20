package org.frangu.customhud.client;

import com.mojang.blaze3d.platform.InputConstants;
import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents;
import net.fabricmc.fabric.api.client.keybinding.v1.KeyBindingHelper;
import net.fabricmc.fabric.api.client.rendering.v1.hud.HudElementRegistry;
import net.fabricmc.fabric.api.client.rendering.v1.hud.HudStatusBarHeightRegistry;
import net.fabricmc.fabric.api.client.rendering.v1.hud.VanillaHudElements;
import net.fabricmc.loader.api.FabricLoader;
import net.minecraft.client.KeyMapping;
import net.minecraft.resources.Identifier;
import org.joml.Matrix3x2fStack;

public class CustomhudClient implements ClientModInitializer {

    public static final String MOD_ID = "customhud";

    @Override
    public void onInitializeClient() {
        CustomHudConfig.load(FabricLoader.getInstance().getConfigDir());

        KeyMapping configKey = KeyBindingHelper.registerKeyBinding(new KeyMapping(
                "key.customhud.open_config",
                InputConstants.Type.KEYSYM,
                InputConstants.KEY_H,
                KeyMapping.Category.MISC
        ));

        ClientTickEvents.END_CLIENT_TICK.register(client -> {
            while (configKey.consumeClick()) {
                client.setScreen(new ConfigScreen(client.screen));
            }
        });

        // Con Fabric API conosciamo l'altezza reale delle barre di stato (cuori extra, barre di altre mod)
        DayHud.statusBarHeight = () -> Math.max(
                HudStatusBarHeightRegistry.getHeight(VanillaHudElements.ARMOR_BAR),
                HudStatusBarHeightRegistry.getHeight(VanillaHudElements.AIR_BAR));

        // Disegnato subito prima delle bossbar vanilla, sotto chat e titoli
        DayHud hud = new DayHud();
        HudElementRegistry.attachElementBefore(VanillaHudElements.BOSS_BAR,
                Identifier.fromNamespaceAndPath(MOD_ID, "day_hud"), hud::render);

        // Sposta le bossbar vanilla sotto il nostro HUD quando è in alto
        HudElementRegistry.replaceElement(VanillaHudElements.BOSS_BAR, original -> (graphics, deltaTracker) -> {
            CustomHudConfig config = CustomHudConfig.INSTANCE;
            if (!config.isHudVisible() || config.positionBottom) {
                original.render(graphics, deltaTracker);
                return;
            }
            Matrix3x2fStack pose = graphics.pose();
            pose.pushMatrix();
            pose.translate(0.0f, DayHud.BOSS_BAR_SHIFT);
            original.render(graphics, deltaTracker);
            pose.popMatrix();
        });
    }
}
