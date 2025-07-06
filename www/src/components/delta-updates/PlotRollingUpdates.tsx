import React from "react"
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend,
} from "chart.js"
import { Bar } from "react-chartjs-2"
import COMPRESSION_FACTORS, { TOTAL_SIZES } from "./data"

ChartJS.register(CategoryScale, LinearScale, BarElement, Title, Tooltip, Legend)

function plotOptions(title: string) {
  return {
    plugins: {
      title: {
        display: true,
        text: title,
        color: "white",
      },
      legend: {
        position: "bottom",
        labels: {
          color: "white",
        },
      },
    },
    responsive: true,
    interaction: {
      mode: "index" as const,
      intersect: false,
    },
    scales: {
      x: {
        stacked: true,
        ticks: {
          color: "white",
        },
      },
      y: {
        stacked: true,
        ticks: {
          color: "white",
        },
      },
    },
  }
}

const labels = ["Monthly", "Quarterly", "Biannually", "Annually"]

const METHODS: Array<{
  label: string
  key: keyof (typeof COMPRESSION_FACTORS)["bookworm"]["annually"]
  color?: string
  stack?: string
}> = [
  { label: "No Delta Updates", key: "compression", color: "#f43f5e" },
  {
    label: "Block-Based, Fixed 4KiB/32KiB",
    key: "block-based-fixed-4-768-32768",
    color: "#c084fc",
  },
  {
    label: "Block-Based, Fixed 4KiB/64KiB",
    key: "block-based-fixed-4-768-65536",
    color: "#a855f7",
  },
  {
    label: "Block-Based, Casync 64KiB",
    key: "block-based-casync-64-768",
    color: "#7dd3fc",
  },
  {
    label: "Block-Based, Casync 16KiB",
    key: "block-based-casync-16-768",
    color: "#0ea5e9",
  },
  {
    label: "File-Based, Deltar 16KiB (Data)",
    key: "deltar-casync-16-768-32768-data",
    color: "#84cc16",
    stack: "deltar-casync-16-768-32768",
  },
  {
    label: "File-Based, Deltar 16KiB (Index)",
    key: "deltar-casync-16-768-32768-plan",
    color: "#d9f99d",
    stack: "deltar-casync-16-768-32768",
  },
  { label: "Xdelta", key: "xdelta", color: "#10b981" },
]

const numUpdates = {"monthly": 12, "quarterly": 4, "biannually": 2, "annually": 1}

const PlotRollingUpdates = () => {
  const data = {
    labels,
    datasets: METHODS.map((method) => {
      return {
        label: method.label,
        data: labels.map(
          (label) =>
            (COMPRESSION_FACTORS["bookworm"][label.toLowerCase()][method.key] /
              COMPRESSION_FACTORS["bookworm"][label.toLowerCase()][
                "compression"
              ] +
              COMPRESSION_FACTORS["bullseye"][label.toLowerCase()][method.key] /
                COMPRESSION_FACTORS["bullseye"][label.toLowerCase()][
                  "compression"
                ]) /
            2,
        ),
        backgroundColor: method.color ?? "rgb(255, 99, 132)",
        stack: method.stack ?? method.key,
      }
    }),
  }
  return (
    <div className="mb-4">
      <Bar options={plotOptions("Minor Updates: Efficiency Ratios")} data={data} />
    </div>
  )
}

export const PlotTotalSizes = () => {
  const data = {
    labels: ["Monthly (12 Updates)", "Quarterly (4 Updates)", "Biannual (2 Updates)", "Annual (1 Update)"],
    datasets: METHODS.map((method) => {
      return {
        label: method.label,
        data: labels.map(
          (label) =>
            (TOTAL_SIZES["bookworm"][label.toLowerCase()][method.key] +
              TOTAL_SIZES["bullseye"][label.toLowerCase()][method.key]) /
            2 / 2 / 1024 / 1024 / 1024,
        ),
        backgroundColor: method.color ?? "rgb(255, 99, 132)",
        stack: method.stack ?? method.key,
      }
    }),
  }
  return (
    <div className="mb-4">
      <Bar options={plotOptions("Minor Updates: Accumulated GiB/Year")} data={data} />
    </div>
  )
}

export const PlotMajorUpdates = () => {
  const data = {
    labels,
    datasets: METHODS.map((method) => {
      return {
        label: method.label,
        data: labels.map(
          (label) =>
            COMPRESSION_FACTORS["bullseye-bookworm"][label.toLowerCase()][method.key] /
              COMPRESSION_FACTORS["bullseye-bookworm"][label.toLowerCase()][
                "compression"
              ],
        ),
        backgroundColor: method.color ?? "rgb(255, 99, 132)",
        stack: method.stack ?? method.key,
      }
    }),
  }
  return (
    <div className="mb-4">
      <Bar options={plotOptions("Major Upgrades")} data={data} />
    </div>
  )
}

export default PlotRollingUpdates
