import * as R from 'ramda';
import React, { useState } from 'react';
import { Input } from 'antd';
import numeral from 'numeral';

import EditableTable from './ItemsTable';
import './App.css';

export interface Item {
  key: string;
  name?: string;
  pricePer?: number | string;
  quantity?: number | string;
  totalValue?: number | string;
}

export const parseAmount = (amountStr: number | string | null | undefined) => {
  if (R.isNil(amountStr)) {
    return 0;
  }

  let s = amountStr.toString();
  if (s.endsWith('k' || s.endsWith('K'))) {
    s = s.replace('k', '').replace('K', '');
    s = (+s * 1000).toString();
  } else if (s.endsWith('m' || s.endsWith('M'))) {
    s = s.replace('m', '').replace('M', '');
    s = (+s * 1000000).toString();
  } else if (s.endsWith('b' || s.endsWith('B'))) {
    s = s.replace('b', '').replace('B', '');
    s = (+s * 1000000000).toString();
  }

  return numeral(s).value();
};

export const formatAmount = (amount: number | string) =>
  numeral(amount).value() > 10000
    ? numeral(amount).format('999.999a')
    : numeral(amount).format('9999');

const App = () => {
  const [items, setItems] = useState<Item[]>([
    {
      key: '0',
      name: 'Superior Dragon Bones',
      pricePer: 32,
      quantity: 4000,
    },
  ]);
  const [username, setUsername] = useState('buyer-username');

  return (
    <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', flex: 1 }}>
      <h1 style={{ paddingTop: '2vh', paddingBottom: 20 }}>OSRSBot Trade Request Generator</h1>
      Username of Buyer
      <Input
        value={username}
        onChange={newVal => setUsername(newVal.target.value)}
        style={{ width: 200, marginBottom: 20 }}
      />
      <EditableTable style={{ minWidth: 800, maxWidth: '80vw', flex: 1 }} onChange={setItems} />
      <div style={{ marginTop: 20, fontSize: 20 }}>
        <b>
          TOTAL:{' '}
          {formatAmount(
            items.reduce(
              (acc, row) =>
                acc +
                (!R.isNil(row.pricePer) && !R.isNil(row.quantity)
                  ? parseAmount(row.pricePer) * parseAmount(row.quantity)
                  : 0),
              0
            )
          )}
        </b>
      </div>
      <code
        style={{
          marginTop: 30,
          minWidth: 800,
          maxWidth: '80vw',
          minHeight: 200,
          color: '#eee',
          padding: 8,
          flex: 1,
          backgroundColor: '#222',
          fontFamily: 'monospace',
        }}
      >
        {items.map(({ name, pricePer, quantity }) => (
          <>
            {`+sellto @${username} ${
              !R.isNil(pricePer) && !R.isNil(quantity)
                ? formatAmount(parseAmount(pricePer) * parseAmount(quantity))
                : '--'
            } ${R.isNil(quantity) ? '--' : formatAmount(quantity)} ${name}`}
            <br />
          </>
        ))}
      </code>
    </div>
  );
};

export default App;
