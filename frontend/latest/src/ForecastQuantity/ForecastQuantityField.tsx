import React from 'react';
import {
  ArrayElement,
  Plugins,
  QueryClientProviderProxy,
  ThemeProviderProxy,
} from '@openmsupply-client/common';
import { usePluginData } from './api';
import { ValueInfoRow } from 'packages/requisitions/src/common';

export type ForecastQuantityFieldPlugin = ArrayElement<
  NonNullable<Plugins['requestRequisitionLine']>['editViewField']
>;

const ForecastQuantityField: ForecastQuantityFieldPlugin = ({
  line,
  unitName,
}) => {
  const { data } = usePluginData.data([line.id]);

  const parsed = JSON.parse(data?.[0]?.data ?? '{}');

  return (
    <ThemeProviderProxy>
      <QueryClientProviderProxy>
        <ValueInfoRow
          key="forecasting-plugin-field"
          value={Math.ceil(parsed?.forecastTotal)}
          label="Forecast Quantity"
          representation="units"
          unitName={unitName}
          defaultPackSize={1}
        />
      </QueryClientProviderProxy>
    </ThemeProviderProxy>
  );
};

export default ForecastQuantityField;
