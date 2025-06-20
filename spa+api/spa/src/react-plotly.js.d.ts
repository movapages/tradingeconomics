declare module 'react-plotly.js' {
  import * as React from 'react';
  import { Layout, Data, Config } from 'plotly.js';

  export interface PlotParams {
    data: Data[];
    layout?: Partial<Layout>;
    config?: Partial<Config>;
    style?: React.CSSProperties;
    className?: string;
    onInitialized?: (figure: any, graphDiv: any) => void;
    onUpdate?: (figure: any, graphDiv: any) => void;
    onPurge?: (figure: any, graphDiv: any) => void;
    useResizeHandler?: boolean;
    [key: string]: any;
  }

  const Plot: React.FC<PlotParams>;
  export default Plot;
} 