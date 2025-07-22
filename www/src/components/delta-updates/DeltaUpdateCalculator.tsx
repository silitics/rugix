import React from "react"

import COMPRESSION_FACTORS from "./data";

const METHODS: Array<{
  name: string
  key: keyof (typeof COMPRESSION_FACTORS)["bookworm"]["annually"]
  tools: string
}> = [
  {
    name: "Block-Based, Fixed 4KiB/32KiB",
    key: "block-based-fixed-4-768-32768",
    tools: "≈RAUC",
  },
  // {
  //   name: "Block-Based, Casync 64KiB",
  //   key: "block-based-casync-64-768",
  //   tools: "Rugix, Casync",
  // },
  {
    name: "Block-Based, Casync 16KiB",
    key: "block-based-casync-16-768",
    tools: "Rugix, Casync",
  },
  {
    name: "File-Based, Deltar 16KiB",
    key: "deltar-casync-16-768-32768",
    tools: "(≈OSTree, ≈APT)",
  },
  {
    name: "Delta Compression (Xdelta)",
    key: "xdelta",
    tools: "Rugix, Mender, Xdelta",
  },
]

const UPDATES_PER_YEAR = {
  monthly: 12,
  quarterly: 4,
  biannually: 2,
  annually: 1,
}

function formatCurrency(value: number) {
  return value.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",")
}

const DeltaUpdateCalculator = () => {
  const [schedule, setSchedule] = React.useState<string>("monthly")
  const [costPerGiB, setCostPerGiB] = React.useState<number>(9)
  const [devicesPower, setDevicesPower] = React.useState<number>(6)
  const [imageSize, setImageSize] = React.useState<number>(1500)

  const devices =
    devicesPower % 2 === 0
      ? Math.pow(10, devicesPower / 2)
      : Math.pow(10, (devicesPower + 1) / 2) / 2
  const baseCost =
    (UPDATES_PER_YEAR[schedule] *
      devices *
      imageSize *
      costPerGiB *
      ((COMPRESSION_FACTORS["bookworm"][schedule]["compression"] +
        COMPRESSION_FACTORS["bullseye"][schedule]["compression"]) /
        2)) /
    1000

  return (
    <div className="space-y-2 mb-7">
      <div className="space-y-1">
        <div className="space-x-2">
          <span>Update Cadence:</span>
          <select
            value={schedule}
            onChange={(e) => setSchedule(e.target.value)}
          >
            <option value="monthly">Monthly</option>
            <option value="quarterly">Quarterly</option>
            <option value="biannually">Biannually</option>
            <option value="annually">Annually</option>
          </select>
        </div>
        <div className="space-x-2 flex">
          <div className="flex-shrink-0">Cost per GiB:</div>
          <input
            type="range"
            min="1"
            max="1000"
            value={costPerGiB}
            className="w-full"
            onChange={(e) => setCostPerGiB(e.target.valueAsNumber)}
          />
          <div className="flex-shrink-0 min-w-[10ch] text-right">
            {(costPerGiB / 100).toFixed(2)} USD
          </div>
        </div>
        <div className="space-x-2 flex">
          <div className="flex-shrink-0">Devices:</div>
          <input
            type="range"
            min="1"
            max="12"
            step="1"
            value={devicesPower}
            className="w-full"
            onChange={(e) => setDevicesPower(e.target.valueAsNumber)}
          />
          <div className="flex-shrink-0 min-w-[10ch] text-right">{devices}</div>
        </div>
        <div className="space-x-2 flex">
          <div className="flex-shrink-0">Image Size:</div>
          <input
            type="range"
            min="10"
            max="15000"
            step="10"
            value={imageSize}
            className="w-full"
            onChange={(e) => setImageSize(parseInt(e.target.value))}
          />
          <div className="flex-shrink-0 min-w-[10ch] text-right">
            {(imageSize / 1000).toFixed(2)} GiB
          </div>
        </div>
      </div>

      <table>
        <thead>
          <th>Method</th>
          <th>Cost (USD/year)</th>
          <th>Savings (USD/year)</th>
          <th>Tools</th>
        </thead>
        <tbody>
          <tr>
            <td>No Delta Updates</td>
            <td className="text-right font-mono">
              {formatCurrency(Math.floor(baseCost / 100))}
            </td>
            <td className="text-right font-mono">0</td>
            <td></td>
          </tr>
          {METHODS.map((method) => {
            const updateCost =
              (UPDATES_PER_YEAR[schedule] *
                devices *
                imageSize *
                costPerGiB *
                ((COMPRESSION_FACTORS["bookworm"][schedule][method.key] +
                  COMPRESSION_FACTORS["bullseye"][schedule][method.key]) /
                  2)) /
              1000
            return (
              <tr key={method.key}>
                <td>{method.name}</td>
                <td className="text-right font-mono">
                  {formatCurrency(Math.floor(updateCost / 100))}
                </td>
                <td className="text-right font-mono font-bold text-green-500">
                  {formatCurrency(Math.floor((baseCost - updateCost) / 100))}
                </td>
                <td>{method.tools}</td>
              </tr>
            )
          })}
        </tbody>
      </table>
    </div>
  )
}

export default DeltaUpdateCalculator
