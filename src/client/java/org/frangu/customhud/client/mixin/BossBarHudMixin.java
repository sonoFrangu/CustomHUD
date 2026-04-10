package org.frangu.customhud.client.mixin;

import net.minecraft.client.gui.DrawContext;
import net.minecraft.client.gui.hud.BossBarHud;
import org.frangu.customhud.client.CustomHudConfig;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

@Mixin(BossBarHud.class)
public class BossBarHudMixin {

    private boolean wasPushed = false;

    @Inject(method = "render", at = @At("HEAD"))
    private void pushBossBarsDown(DrawContext context, CallbackInfo ci) {
        CustomHudConfig config = CustomHudConfig.INSTANCE;

        // Controlliamo se l'HUD è acceso e posizionato in ALTO
        boolean isHudActive = config.showDayCounter || config.showClock || config.showBossbar;
        boolean isTop = !config.positionBottom;

        if (isHudActive && isTop) {
            context.getMatrices().pushMatrix();
            // Spostiamo la bossbar VANILLA in giù di 35 pixel
            context.getMatrices().translate(0.0f, 35.0f);
            wasPushed = true;
        } else {
            wasPushed = false;
        }
    }

    @Inject(method = "render", at = @At("RETURN"))
    private void popBossBars(DrawContext context, CallbackInfo ci) {
        // Se l'avevamo spinta in giù, chiudiamo la matrice
        if (wasPushed) {
            context.getMatrices().popMatrix();
        }
    }
}