import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";

import style from "../styles/Header.module.css";

const Header = () => {
  return (
    <div className={style.wrapper}>
      <div className="bg-blue text-3xl font-bold text-blue-300">Lottery Dapp 💰</div>
      {/* <button className="bg-blue-300 p-2 px-3 rounded-md text-black font-bold">Connect Wallet</button> */}
      <WalletMultiButton/>
    </div>
  );
};

export default Header;
