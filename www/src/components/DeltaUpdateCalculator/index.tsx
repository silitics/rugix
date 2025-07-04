import React from "react"

const COMPRESSION_FACTORS = {
  bookworm: {
    annually: {
      compression: 0.177588749791722,
      xdelta: 0.05759843534519772,
      "block-based-fixed-4-512-32768": 0.13464109434036994,
      "block-based-casync-64-512": 0.13078671377618925,
      "block-based-casync-8-512": 0.13780739196091338,
      "deltar-casync-16-1024-32768": 0.12287954705426632,
      "deltar-fixed-4096-0-1": 0.1134245771615329,
    },
    monthly: {
      compression: 0.17755644920078015,
      xdelta: 0.02505955081548573,
      "block-based-fixed-4-512-32768": 0.053336596160729235,
      "block-based-casync-64-512": 0.04876413177051478,
      "block-based-casync-8-512": 0.0497717878316955,
      "deltar-casync-16-1024-32768": 0.04650260369061429,
      "deltar-fixed-4096-0-1": 0.04501082227218963,
    },
    quarterly: {
      compression: 0.1775700547076022,
      xdelta: 0.04042226455276389,
      "block-based-fixed-4-512-32768": 0.09421960465281733,
      "block-based-casync-64-512": 0.09022043653108476,
      "block-based-casync-8-512": 0.09072419800435477,
      "deltar-casync-16-1024-32768": 0.08482071466243937,
      "deltar-fixed-4096-0-1": 0.08193599483792527,
    },
    biannually: {
      compression: 0.17756723152867576,
      xdelta: 0.050177143845317615,
      "block-based-fixed-4-512-32768": 0.11675431167200062,
      "block-based-casync-64-512": 0.11265902203653574,
      "block-based-casync-8-512": 0.11568107459293851,
      "deltar-casync-16-1024-32768": 0.10613234577874603,
      "deltar-fixed-4096-0-1": 0.10076456805642212,
    },
  },
}

const METHODS: Array<{
  name: string
  key: keyof (typeof COMPRESSION_FACTORS)["bookworm"]["annually"]
  tools: string
}> = [
  {
    name: "Block-Based, Fixed 4KiB",
    key: "block-based-fixed-4-512-32768",
    tools: "RAUC",
  },
  {
    name: "Block-Based, Casync 64KiB",
    key: "block-based-casync-64-512",
    tools: "Rugix, Casync",
  },
  {
    name: "File-Based, Deltar 16KiB",
    key: "deltar-casync-16-1024-32768",
    tools: "(OSTree, APT)",
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

const DeltaUpdateCalculator = () => {
  const [schedule, setSchedule] = React.useState<string>("monthly")
  const [costPerGiB, setCostPerGiB] = React.useState<number>(9)
  const [devicesPower, setDevicesPower] = React.useState<number>(3)
  const [imageSize, setImageSize] = React.useState<number>(1500)

  const devices = Math.pow(10, devicesPower);
  const baseCost =
    (UPDATES_PER_YEAR[schedule] *
      devices *
      imageSize *
      costPerGiB *
      COMPRESSION_FACTORS["bookworm"][schedule]["compression"]) /
    1000

  return (
    <div className="space-y-2 mb-7">
      <div className="space-y-1">
        <div className="space-x-2">
          <span>Update Schedule:</span>
          <select value={schedule} onChange={(e) => setSchedule(e.target.value)}>
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
            max="2000"
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
            max="6"
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
            min="1"
            max="15000"
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
              {(baseCost / 100).toFixed(0)}
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
                COMPRESSION_FACTORS["bookworm"][schedule][method.key]) /
              1000
            return (
              <tr key={method.key}>
                <td>{method.name}</td>
                <td className="text-right font-mono">
                  {(updateCost / 100).toFixed(0)}{" "}
                </td>
                <td className="text-right font-mono font-bold text-green-500">
                  {((baseCost - updateCost) / 100).toFixed(0)}
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
