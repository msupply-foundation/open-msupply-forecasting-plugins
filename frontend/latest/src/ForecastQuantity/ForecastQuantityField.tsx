import React from 'react';
import {
  ArrayElement,
  BasicTextInput,
  InputWithLabelRow,
  Plugins,
  QueryClientProviderProxy,
  ThemeProviderProxy,
} from '@openmsupply-client/common';
import { usePluginData } from './api';

export type ForecastQuantityFieldPlugin = ArrayElement<
  NonNullable<Plugins['requestRequisitionLine']>['editViewField']
>;

const ForecastQuantityField: ForecastQuantityFieldPlugin = ({ line }) => {
  const { data } = usePluginData.data([line.id]);

  const parsed = JSON.parse(data?.[0]?.data ?? '{}');

  return (
    <ThemeProviderProxy>
      <QueryClientProviderProxy>
        <InputWithLabelRow
          label={'Forecast Quantity'}
          labelWidth="200px"
          sx={{
            justifyContent: 'space-between',
            '.MuiFormControl-root > .MuiInput-root > input': {
              maxWidth: '120px',
              textAlign: 'right',
            },
          }}
          Input={
            <BasicTextInput
              value={String(Math.ceil(parsed?.forecastTotal))}
              disabled
            />
          }
        />
      </QueryClientProviderProxy>
    </ThemeProviderProxy>
  );
};

export default ForecastQuantityField;
