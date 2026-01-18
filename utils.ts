import * as anchor from "@coral-xyz/anchor";

export const getProvider = () => {
  return anchor.AnchorProvider.env();
};
