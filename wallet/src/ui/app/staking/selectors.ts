import { isSuiMoveObject, isSuiObject } from '@mysten/sui.js';
import { createSelector } from '@reduxjs/toolkit';

import { ownedObjects } from '_redux/slices/account';
import { SUI_SYSTEM_STATE_OBJECT_ID } from '_redux/slices/sui-objects/Coin';

import type { SuiAddress, SuiData, SuiMoveObject } from '@mysten/sui.js';
import { RootState } from '../redux/RootReducer';
import { suiObjectsAdapterSelectors } from '../redux/slices/sui-objects';

export const delegationsSelector = createSelector(ownedObjects, (objects) =>
    objects.filter(
        ({ data }) =>
            isSuiMoveObject(data) && data.type === DELEGATION_OBJECT_TYPE
    )
);

export const activeDelegationsSelector = createSelector(
    delegationsSelector,
    (delegations) =>
        delegations.filter(
            ({ data }) =>
                isDelegationData(data) &&
                data.fields.active_delegation > 0 &&
                data.fields.ending_epoch.fields.vec === ''
        )
);

export const activeDelegationIDsSelector = createSelector(
    activeDelegationsSelector,
    (delegations) => delegations.map(({ reference: { objectId } }) => objectId)
);

export const totalActiveStakedSelector = createSelector(
    activeDelegationsSelector,
    (activeDelegations) =>
        activeDelegations.reduce((total, { data }) => {
            if (isDelegationData(data)) {
                total += BigInt(data.fields.active_delegation);
            }
            return total;
        }, BigInt(0))
);

export const epochSelector = (state: RootState) => {
    const { data } =
        suiObjectsAdapterSelectors.selectById(
            state,
            SUI_SYSTEM_STATE_OBJECT_ID
        ) || {};
    console.log(data, isSuiMoveObject(data));
    return isSuiMoveObject(data) ? (data.fields.epoch as number) : null;
};
