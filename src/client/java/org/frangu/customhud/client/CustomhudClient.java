package org.frangu.customhud.client;

import com.mojang.blaze3d.platform.InputConstants;
import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents;
import net.fabricmc.fabric.api.client.keymapping.v1.KeyMappingHelper;
import net.fabricmc.fabric.api.client.rendering.v1.hud.HudElementRegistry;
import net.fabricmc.fabric.api.client.rendering.v1.hud.VanillaHudElements;
import net.minecraft.client.KeyMapping;
import net.minecraft.resources.Identifier;
import org.joml.Matrix3x2fStack;

public class CustomhudClient implements ClientModInitializer {

    public static final String MOD_ID = "customhud";

    @Override
    public void onInitializeClient() {
        CustomHudConfig.load();

        KeyMapping configKey = KeyMappingHelper.registerKeyMapping(new KeyMapping(
                "key.customhud.open_config",
                InputConstants.Type.KEYBOARD,
                InputConstants.KEY_H,
                KeyMapping.Category.MISC
        ));

        ClientTickEvents.END_CLIENT_TICK.register(client -> {
            while (configKey.consumeClick()) {
                client.gui.setScreen(new ConfigScreen(client.gui.screen()));
            }
        });

        // Disegnato subito prima delle bossbar vanilla, sotto chat e titoli
        HudElementRegistry.attachElementBefore(VanillaHudElements.BOSS_BAR,
                Identifier.fromNamespaceAndPath(MOD_ID, "day_hud"), new DayHudElement());

        // Sposta le bossbar vanilla sotto il nostro HUD quando è in alto (sostituisce il vecchio mixin)
        HudElementRegistry.replaceElement(VanillaHudElements.BOSS_BAR, original -> (graphics, deltaTracker) -> {
            CustomHudConfig config = CustomHudConfig.INSTANCE;
            if (!config.isHudVisible() || config.positionBottom) {
                original.extractRenderState(graphics, deltaTracker);
                return;
            }
            Matrix3x2fStack pose = graphics.pose();
            pose.pushMatrix();
            pose.translate(0.0f, DayHudElement.BOSS_BAR_SHIFT);
            original.extractRenderState(graphics, deltaTracker);
            pose.popMatrix();
        });
    }
}
