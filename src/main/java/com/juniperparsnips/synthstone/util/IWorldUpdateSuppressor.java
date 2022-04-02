package com.juniperparsnips.synthstone.util;

public interface IWorldUpdateSuppressor {

    boolean getShouldPreventBlockUpdates();

    void setShouldPreventBlockUpdates(boolean preventUpdates);

}
