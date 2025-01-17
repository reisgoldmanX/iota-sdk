// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

import com.google.gson.*;
import com.google.gson.annotations.JsonAdapter;

import java.lang.reflect.Type;

@JsonAdapter(BaseCoinBalanceAdapter.class)
public class BaseCoinBalance extends AbstractObject {
    /// Total amount
    private long total;
    /// Balance that can currently be spent
    private long available;
    /// Voting power
    private long votingPower;

    public BaseCoinBalance(long total, long available, long votingPower) {
        this.total = total;
        this.available = available;
    }

    public long getTotal() {
        return total;
    }

    public long getAvailable() {
        return available;
    }

    public long getVotingPower() {
        return votingPower;
    }
}

class BaseCoinBalanceAdapter implements JsonDeserializer<BaseCoinBalance>, JsonSerializer<BaseCoinBalance> {
    @Override
    public BaseCoinBalance deserialize(final JsonElement json, final Type typeOfT, final JsonDeserializationContext context)
            throws JsonParseException {

        long total = Long.parseLong(json.getAsJsonObject().get("total").getAsString());
        long available = Long.parseLong(json.getAsJsonObject().get("available").getAsString());
        long votingPower = Long.parseLong(json.getAsJsonObject().get("votingPower").getAsString());

        return new BaseCoinBalance(total, available, votingPower);
    }

    public JsonElement serialize(BaseCoinBalance src, Type typeOfSrc, JsonSerializationContext context) {
        JsonObject o = new JsonObject();
        
        o.addProperty("total", String.valueOf(src.getTotal()));
        o.addProperty("available", String.valueOf(src.getAvailable()));
        o.addProperty("votingPower", String.valueOf(src.getVotingPower()));
        
        return o;
    }
}