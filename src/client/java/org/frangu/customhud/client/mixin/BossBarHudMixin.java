package org.frangu.customhud.client.mixin;

import net.minecraft.client.gui.DrawContext;
import net.minecraft.client.gui.hud.BossBarHud;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

@Mixin(BossBarHud.class)
public class BossBarHudMixin {

    @Inject(method = "render", at = @At("HEAD"))
    private void pushBossBarsDown(DrawContext context, CallbackInfo ci) {
        context.getMatrices().pushMatrix();
        // SOLO X e Y: Spostiamo la barra in giù di 35 pixel
        context.getMatrices().translate(0.0f, 35.0f);
    }

    @Inject(method = "render", at = @At("RETURN"))
    private void popBossBars(DrawContext context, CallbackInfo ci) {
        context.getMatrices().popMatrix();
    }
}