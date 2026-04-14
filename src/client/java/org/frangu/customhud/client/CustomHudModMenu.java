package org.frangu.customhud.client;

import com.terraformersmc.modmenu.api.ConfigScreenFactory;
import com.terraformersmc.modmenu.api.ModMenuApi;

public class CustomHudModMenu implements ModMenuApi {

    @Override
    public ConfigScreenFactory<?> getModConfigScreenFactory() { // <-- Aggiunto "Mod" qui!
        return parent -> new ConfigScreen(parent);
    }
}