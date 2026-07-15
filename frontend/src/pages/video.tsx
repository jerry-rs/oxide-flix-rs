import { useEffect, useState } from "react"
import { useParams, useLocation } from "react-router-dom"
import { Play } from "lucide-react"

type VideoDetailResponse = {
  filename?: string
  size_bytes?: number
  mime_type?: string
  sha256?: string
  suggested_player_mode?: string
}

const formatSize = (sizeBytes?: number) => {
  if (sizeBytes === undefined || sizeBytes === null) return "unknown"
  if (sizeBytes < 1024) return `${sizeBytes} B`
  if (sizeBytes < 1024 * 1024) return `${(sizeBytes / 1024).toFixed(1)} KB`
  if (sizeBytes < 1024 * 1024 * 1024) return `${(sizeBytes / (1024 * 1024)).toFixed(1)} MB`
  return `${(sizeBytes / (1024 * 1024 * 1024)).toFixed(1)} GB`
}

function VideoPage() {
  const { videoName } = useParams<{ videoName: string }>()
  const location = useLocation()
  const decodedName = videoName ? decodeURIComponent(videoName) : ""
  const [detail, setDetail] = useState<VideoDetailResponse | null>(null)
  const [loading, setLoading] = useState(false)

  const searchParams = new URLSearchParams(location.search)
  const currentPath = location.state?.currentPath ?? searchParams.get("currentPath") ?? ""
  const normalizedPath = currentPath.replace(/^\/+|\/+$/g, "")
  const encodedPath = normalizedPath
    ? normalizedPath.split("/").filter(Boolean).map(encodeURIComponent).join("/")
    : ""
  const fullPath = encodedPath ? `${encodedPath}/${encodeURIComponent(decodedName)}` : encodeURIComponent(decodedName)
  const streamUrl = decodedName ? `/api/video/stream/${fullPath}` : ""

  useEffect(() => {
    const loadDetail = async () => {
      if (!decodedName) return

      setLoading(true)
      try {
        const response = await fetch(`/api/video/info/${fullPath}`)
        const data = await response.json()
        setDetail(data)
      } catch (error) {
        console.error("Failed to load video detail", error)
      } finally {
        setLoading(false)
      }
    }

    loadDetail()
  }, [decodedName, fullPath])

  return (
    <div className="min-h-screen bg-slate-950 text-white">
      <div className="mx-auto flex max-w-5xl flex-col gap-6 px-6 py-8">
        <div className="rounded-2xl border border-slate-800 bg-slate-900 p-8 shadow-2xl">
          <div className="flex items-center gap-3">
            <div className="flex h-14 w-14 items-center justify-center rounded-full bg-emerald-500/20 text-emerald-400">
              <Play className="h-6 w-6" />
            </div>
            <div>
              <p className="text-sm text-emerald-400">Video detail</p>
              <h1 className="text-2xl font-semibold">{decodedName || "Untitled video"}</h1>
            </div>
          </div>

          <div className="mt-8 flex flex-col gap-6">
            <div className="rounded-2xl border border-slate-800 bg-slate-950 p-6 shadow-xl">
              <div className="mb-4">
                <p className="text-sm text-emerald-400">Details</p>
                <h2 className="text-lg font-semibold text-white">Video information</h2>
              </div>

              {loading ? (
                <p className="text-slate-400">Loading video details...</p>
              ) : detail ? (
                <div className="space-y-3 text-slate-300">
                  <p className="break-all">
                    <span className="text-slate-500">Filename:</span> {detail.filename ?? decodedName}
                  </p>
                  <div className="grid gap-3 md:grid-cols-2">
                    <p><span className="text-slate-500">Size:</span> {formatSize(detail.size_bytes)}</p>
                    <p><span className="text-slate-500">Mime Type:</span> {detail.mime_type ?? "unknown"}</p>
                    <p className="break-all md:col-span-2"><span className="text-slate-500">SHA256:</span> {detail.sha256 ?? "unknown"}</p>
                    <p className="md:col-span-2"><span className="text-slate-500">Player Mode:</span> {detail.suggested_player_mode ?? "unknown"}</p>
                  </div>
                </div>
              ) : (
                <p className="text-slate-400">No video detail available.</p>
              )}
            </div>

            {streamUrl ? (
              <div className="rounded-2xl border border-slate-800 bg-slate-950 p-6 shadow-xl">
                <div className="mb-4">
                  <p className="text-sm text-emerald-400">Preview</p>
                  <h2 className="text-lg font-semibold text-white">Video player</h2>
                </div>
                <video
                  controls
                  playsInline
                  preload="metadata"
                  className="w-full rounded-xl border border-slate-800 bg-black"
                  style={{ minHeight: "420px", maxHeight: "640px", objectFit: "contain" }}
                  src={streamUrl}
                />
              </div>
            ) : null}
          </div>
        </div>
      </div>
    </div>
  )
}

export default VideoPage


