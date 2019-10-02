import React, { FC } from "react";

import { Pc, Mobile, Tablet } from "app/components/responsive";

import MobileComponent from "./mobile";
import PcComponent from "./pc";

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
