import React, {FC, useState, useContext} from 'react';
import styled from 'styled-components';

enum Device {
  Mobile,
  Tablet,
  Pc,
}

const DeviceContext = React.createContext<Device>(Device.Mobile);

export const Responsive: FC<{}> = ({children}) => {
  const [device, setDevice] = useState(Device.Mobile);

  window.onresize = () => {
    const width = window.parent.screen.width;
    if (width < 768) {
      if (device !== Device.Mobile) {
        setDevice(Device.Mobile);
      }
    } else if (width < 980) {
      if (device !== Device.Tablet) {
        setDevice(Device.Tablet);
      }
    } else if (device != Device.Pc) {
      setDevice(Device.Pc);
    }
  };

  return (
    <DeviceContext.Provider value={device}>{children}</DeviceContext.Provider>
  );
};

interface ContentProps {
  children: React.ReactNode;
}

const genDeviceContent: (device: Device) => FC<ContentProps> = device => ({
  children,
}) => {
  const curDevice = useContext(DeviceContext);

  console.log(curDevice);

  return <>{curDevice === device ? children : null}</>;
};

export const Pc: FC<ContentProps> = React.memo(genDeviceContent(Device.Pc));
export const Tablet: FC<ContentProps> = React.memo(
  genDeviceContent(Device.Tablet),
);
export const Mobile: FC<ContentProps> = React.memo(
  genDeviceContent(Device.Mobile),
);
