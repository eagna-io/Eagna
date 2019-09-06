import React, { FC } from "react";

import { Pc, Mobile, Tablet } from "app/components/responsive";

import MobileComponent from "./tokens/mobile";
import PcComponent from "./tokens/pc";

const TokenListComponent: FC = () => {
  return (
    <>
      <Mobile>
        <MobileComponent />
      </Mobile>
      <Tablet>
        <MobileComponent />
      </Tablet>
      <Pc>
        <PcComponent />
      </Pc>
    </>
  );
};

export default TokenListComponent;
