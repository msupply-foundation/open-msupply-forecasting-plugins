import React from 'react';
import {
  ArrayElement,
  Plugins,
  QueryClientProviderProxy,
  ThemeProviderProxy,
  UNDEFINED_STRING_VALUE,
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
  const value =
    parsed?.forecastTotal === null ? null : Math.ceil(parsed?.forecastTotal);

  return (
    <ThemeProviderProxy>
      <QueryClientProviderProxy>
        <ValueInfoRow
          key="forecasting-plugin-field"
          value={value}
          label="Forecast Quantity"
          representation="units"
          unitName={unitName}
          defaultPackSize={1}
          nullDisplay={UNDEFINED_STRING_VALUE}
        />
      </QueryClientProviderProxy>
    </ThemeProviderProxy>
  );
};

export default ForecastQuantityField;
