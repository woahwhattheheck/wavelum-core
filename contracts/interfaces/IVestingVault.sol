// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/**
 * @title IVestingVault
 * @dev Interface for the EVM vesting vault contract.
 *
 * Run `scripts/sync-interfaces.sh --check` after changing Soroban
 * #[contractimpl] entrypoints or this Solidity interface. Known legacy EVM
 * drift must be documented with a custom sync-allow tag.
 */
interface IVestingVault {
    struct Grant {
        uint256 amount;
        uint256 start;
        uint256 duration;
        uint256 claimed;
        bool isActive;
        bool isEscrowed; // New field for frozen tokens
        uint256 escrowed_amount; // amount currently held in escrow for this grant
        // Tax configuration
        uint16 tax_bps; // basis points, parts per 10,000
        address tax_authority; // address receiving withheld tax
        address tax_asset; // if non-zero and different from vested token, tax portion will be liquidated to this asset
        uint256 cumulative_taxes_paid; // accumulated taxes paid (in tax_asset units when liquidated, otherwise token units)
        uint256 tax_rounding_accumulator; // accumulator for fractional tax parts to avoid losing stroops
    }

    /**
     * @dev Claims vested tokens for a beneficiary.
     * @param beneficiary The address claiming tokens.
     * @custom:sync-allow legacy-evm Soroban claim currently requires user, vesting_id, and amount.
     */
    function claim(address beneficiary) external;

    /**
     * @dev Gets the claimable amount for a beneficiary.
     * @param beneficiary The address to check.
     * @return The amount of tokens that can be claimed.
     * @custom:sync-allow legacy-evm No Soroban getter currently exposes this exact EVM vesting calculation.
     */
    function getClaimableAmount(address beneficiary) external view returns (uint256);

    /**
     * @dev Gets the grant details for a beneficiary.
     * @param beneficiary The address to check.
     * @return The grant details.
     * @custom:sync-allow legacy-evm Soroban uses get_vesting_grant_info keyed by vesting_id.
     */
    function getGrant(address beneficiary) external view returns (Grant memory);

    /**
     * @dev Emitted when tokens are claimed.
     */
    event TokensClaimed(address indexed beneficiary, uint256 amount);

    /**
     * @dev Emitted when tokens are frozen due to sanctions.
     */
    event TokensFrozen(address indexed beneficiary, uint256 amount);

    /**
     * @dev Emitted when tokens are released from escrow.
     */
    event TokensReleased(address indexed beneficiary, uint256 amount);

    /**
     * @dev Emitted when tax is withheld from a claim and sent to authority.
     */
    event TaxWithheld(address indexed beneficiary, uint256 gross, uint256 taxAmount, uint256 net);

    /**
     * @dev Emitted when the KPI vesting multiplier changes.
     */
    event KPIMultiplierUpdated(uint256 oldMultiplier, uint256 oracleInput, uint256 newMultiplier, uint256 timestamp);
}
