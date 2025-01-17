// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package org.iota.types;

public class OutputParams extends AbstractObject {

    private String recipientAddress;
    private String amount;
    private Assets assets;
    private Features features;
    private Unlocks unlocks;
    private StorageDeposit storageDeposit;

    public static class Assets extends AbstractObject {
        private NativeToken[] nativeTokens;
        private String nftId;

        public NativeToken[] getNativeTokens() {
            return nativeTokens;
        }

        public Assets withNativeTokens(NativeToken[] nativeTokens) {
            this.nativeTokens = nativeTokens;
            return this;
        }

        public String getNftId() {
            return nftId;
        }

        public Assets withNftId(String nftId) {
            this.nftId = nftId;
            return this;
        }
    }

    public static class Features extends AbstractObject {
        private String tag;
        private String metadata;

        public String getTag() {
            return tag;
        }

        public Features withTag(String tag) {
            this.tag = tag;
            return this;
        }

        public String getMetadata() {
            return metadata;
        }

        public Features withMetadata(String metadata) {
            this.metadata = metadata;
            return this;
        }
    }

    public static class Unlocks extends AbstractObject {
        private Integer expirationUnixTime;
        private Integer timelockUnixTime;

        public Integer getExpirationUnixTime() {
            return expirationUnixTime;
        }

        public Unlocks withExpirationUnixTime(Integer expirationUnixTime) {
            this.expirationUnixTime = expirationUnixTime;
            return this;
        }

        public Integer getTimelockUnixTime() {
            return timelockUnixTime;
        }

        public Unlocks withTimelockUnixTime(Integer timelockUnixTime) {
            this.timelockUnixTime = timelockUnixTime;
            return this;
        }
    }

    public static class StorageDeposit extends AbstractObject {
        private ReturnStrategy returnStrategy;
        // If account has 2 Mi, min storage deposit is 1 Mi and one wants to send 1.5
        // Mi, it wouldn't be possible with a
        // 0.5 Mi remainder. To still send a transaction, the 0.5 can be added to the
        // output automatically, if set to true
        private boolean useExcessIfLow;

        public ReturnStrategy getReturnStrategy() {
            return returnStrategy;
        }

        public StorageDeposit withReturnStrategy(ReturnStrategy returnStrategy) {
            this.returnStrategy = returnStrategy;
            return this;
        }

        public boolean isUseExcessIfLow() {
            return useExcessIfLow;
        }

        public StorageDeposit withUseExcessIfLow(boolean useExcessIfLow) {
            this.useExcessIfLow = useExcessIfLow;
            return this;
        }
    }

    public enum ReturnStrategy {
        // A storage deposit return unlock condition will be added with the required
        // minimum storage deposit
        Return,
        // The recipient address will get the additional amount to reach the minimum
        // storage deposit gifted
        Gift,
    }

    public String getRecipientAddress() {
        return recipientAddress;
    }

    public OutputParams withRecipientAddress(String recipientAddress) {
        this.recipientAddress = recipientAddress;
        return this;
    }

    public String getAmount() {
        return amount;
    }

    public OutputParams withAmount(String amount) {
        this.amount = amount;
        return this;
    }

    public Assets getAssets() {
        return assets;
    }

    public OutputParams withAssets(Assets assets) {
        this.assets = assets;
        return this;
    }

    public Features getFeatures() {
        return features;
    }

    public OutputParams withFeatures(Features features) {
        this.features = features;
        return this;
    }

    public Unlocks getUnlocks() {
        return unlocks;
    }

    public OutputParams withUnlocks(Unlocks unlocks) {
        this.unlocks = unlocks;
        return this;
    }

    public StorageDeposit getStorageDeposit() {
        return storageDeposit;
    }

    public OutputParams withStorageDeposit(StorageDeposit storageDeposit) {
        this.storageDeposit = storageDeposit;
        return this;
    }
}
