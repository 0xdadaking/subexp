import React, { useEffect, useState } from 'react'
import { Form, Grid } from 'semantic-ui-react'

import { useSubstrateState } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

import KittyCards from './KittyCards'

const parseKitty = ({ dna, price, gender, owner }) => ({
  dna,
  price: price.toJSON(),
  gender: gender.toJSON(),
  owner: owner.toJSON(),
})

function toHexString(byteArray) {
  var s = '0x'
  byteArray.forEach(function (byte) {
    s += ('0' + (byte & 0xff).toString(16)).slice(-2)
  })
  return s
}

export default function Kitties(props) {
  const { api, keyring } = useSubstrateState()
  const [kittyIds, setKittyIds] = useState([])
  const [kitties, setKitties] = useState([])
  const [status, setStatus] = useState('')

  const subscribeCount = () => {
    let unsub = null

    const asyncFetch = async () => {
      unsub = await api.query.substrateKitties.countForKitties(async count => {
        // Fetch all kitty keys
        const entries = await api.query.substrateKitties.kitties.entries()
        const ids = entries.map(entry => toHexString(entry[0].slice(-32)))
        setKittyIds(ids)
      })
    }

    asyncFetch()

    return () => {
      unsub && unsub()
    }
  }

  const subscribeKitties = () => {
    let unsub = null

    const asyncFetch = async () => {
      unsub = await api.query.substrateKitties.kitties.multi(
        kittyIds,
        kitties => {
          const kittiesMap = kitties.map(kitty => {
            //FIXME! 
            if (kitty.value.isSome())
              return parseKitty(kitty.unwrap());
            else
              return null;
          })
          setKitties(kittiesMap)
        }
      )
    }

    asyncFetch()

    return () => {
      unsub && unsub()
    }
  }

  useEffect(subscribeCount, [api, keyring])
  useEffect(subscribeKitties, [api, keyring, kittyIds])

  return (
    <Grid.Column width={16}>
      <h1>Kitties</h1>
      <KittyCards kitties={kitties} setStatus={setStatus} />
      <Form style={{ margin: '1em 0' }}>
        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            label="Create Kitty"
            type="SIGNED-TX"
            setStatus={setStatus}
            attrs={{
              palletRpc: 'substrateKitties',
              callable: 'createKitty',
              inputParams: [],
              paramFields: [],
            }}
          />
        </Form.Field>
      </Form>
      <div style={{ overflowWrap: 'break-word' }}>{status}</div>
    </Grid.Column>
  )
}
