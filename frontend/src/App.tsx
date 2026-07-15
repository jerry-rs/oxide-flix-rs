import { useEffect, useRef, useState } from "react"
import {
  createColumnHelper,
  flexRender,
  getCoreRowModel,
  useReactTable,
} from "@tanstack/react-table"
import { Folder, Home, Video } from "lucide-react"
import { InputOTP, InputOTPGroup, InputOTPSeparator, InputOTPSlot } from "./components/ui/input-otp"

type VideoEntry = {
  name: string
  is_dir: boolean
  size_bytes: number
}

const columnHelper = createColumnHelper<VideoEntry>()

const formatSize = (sizeBytes: number) => {
  if (sizeBytes < 1024) return `${sizeBytes} B`
  if (sizeBytes < 1024 * 1024) return `${(sizeBytes / 1024).toFixed(1)} KB`
  return `${(sizeBytes / (1024 * 1024)).toFixed(1)} MB`
}

const columns = [
  columnHelper.accessor("name", {
    header: "Name",
    cell: (info) => {
      const isDir = info.row.original.is_dir

      return (
        <div className="flex items-center gap-3">
          <div className={`flex h-10 w-10 items-center justify-center rounded-lg text-sm font-semibold ${isDir ? "bg-sky-500/15 text-sky-400" : "bg-emerald-500/15 text-emerald-400"}`}>
            {isDir ? <Folder className="h-5 w-5" /> : <Video className="h-5 w-5" />}
          </div>
          <div>
            <p className="font-medium text-white">{info.getValue()}</p>
            <p className="text-sm text-slate-400">{isDir ? "文件夹" : "文件"}</p>
          </div>
        </div>
      )
    },
  }),
  columnHelper.accessor("is_dir", {
    header: "Type",
    cell: (info) => <span className="text-slate-300">{info.getValue() ? "文件夹" : "文件"}</span>,
  }),
  columnHelper.accessor("size_bytes", {
    header: "Size",
    cell: (info) => <span className="text-slate-300">{formatSize(info.getValue())}</span>,
  }),
]

type VideoListResponse = {
  entries: VideoEntry[]
  current_path: string
}

function App() {
  const [hasAuth, setHasAuth] = useState(false)
  const [otp, setOtp] = useState("")
  const [videoData, setVideoData] = useState<VideoEntry[]>([])
  const [currentPath, setCurrentPath] = useState("")
  const [loading, setLoading] = useState(false)
  const fileInputRef = useRef<HTMLInputElement | null>(null)

  const loadVideos = async (path = "") => {
    setLoading(true)
    try {
      const normalizedPath = path.replace(/^\/+|\/+$/g, "")
      const encodedPath = normalizedPath
        ? normalizedPath.split("/").filter(Boolean).map(encodeURIComponent).join("/")
        : ""
      const url = encodedPath ? `/api/video/list/${encodedPath}` : "/api/video/list"
      const response = await fetch(url)
      const data: VideoListResponse = await response.json()
      setVideoData(data.entries)
      setCurrentPath(data.current_path)
    } catch (error) {
      console.error("Failed to load videos", error)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    if (!hasAuth) return
    loadVideos()
  }, [hasAuth])

  const handleOpenFolder = (folderName: string) => {
    loadVideos(folderName)
  }

  const handleOpenVideo = (videoName: string) => {
    const pathParam = currentPath ? `?currentPath=${encodeURIComponent(currentPath)}` : ""
    const url = `/video/${encodeURIComponent(videoName)}${pathParam}`
    const fullUrl = `${window.location.origin}${url}`
    window.open(fullUrl, "_blank", "noopener,noreferrer")
  }

  const handleBreadcrumbClick = (segments: string[]) => {
    const targetPath = segments.join("/")
    loadVideos(targetPath)
  }

  const handleUploadClick = () => {
    fileInputRef.current?.click()
  }

  const handleFileChange = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const files = Array.from(event.target.files ?? [])
    if (!files.length) return

    const uploadPath = currentPath ? `/${currentPath.replace(/^\/+|\/+$/g, "")}` : ""
    const uploadUrl = uploadPath ? `/api/video/upload${uploadPath}` : "/api/video/upload"

    for (const file of files) {
      const formData = new FormData()
      formData.append("file", file)

      try {
        await fetch(uploadUrl, {
          method: "POST",
          body: formData,
        })
      } catch (error) {
        console.error("Upload failed", error)
      }
    }

    loadVideos(currentPath)
    event.target.value = ""
  }

  const table = useReactTable({
    data: videoData,
    columns,
    getCoreRowModel: getCoreRowModel(),
  })

  return (
    <>
      {hasAuth ? (
        <div className="min-h-screen bg-slate-950 text-white">
          <nav className="border-b border-slate-800 bg-slate-900/80 backdrop-blur">
            <div className="mx-auto flex max-w-6xl items-center justify-between px-6 py-4">
              <div className="flex items-center gap-3">
                <div className="flex h-10 w-10 items-center justify-center rounded-full bg-emerald-500/20 text-emerald-400">
                  <Video className="h-5 w-5" />
                </div>
                <div>
                  <p className="font-semibold font-mono font-size-xl">OXIDE FLIX</p>
                </div>
              </div>
    
            </div>
          </nav>

          <main className="mx-auto flex max-w-6xl flex-col gap-6 px-6 py-8">
            <nav aria-label="Breadcrumb" className="text-sm text-slate-400">
              <ol className="flex items-center gap-2">
                <li className="flex items-center gap-2">
                  <button
                    type="button"
                    className="flex items-center gap-2 hover:text-white"
                    onClick={() => loadVideos("")}
                  >
                    <Home className="h-4 w-4" />
                    <span className="text-white">Home</span>
                  </button>
                </li>
                {currentPath
                  ? currentPath
                      .split("/")
                      .filter(Boolean)
                      .map((segment, index, arr) => {
                        const pathSegments = arr.slice(0, index + 1)
                        const isLast = index === arr.length - 1

                        return (
                          <li key={pathSegments.join("/")} className="flex items-center gap-2">
                            <span>/</span>
                            {!isLast ? (
                              <button
                                type="button"
                                className="hover:text-white"
                                onClick={() => handleBreadcrumbClick(pathSegments)}
                              >
                                {segment}
                              </button>
                            ) : (
                              <span className="text-white">{segment}</span>
                            )}
                          </li>
                        )
                      })
                  : (
                  <></>
                  )}
              </ol>
            </nav>

            <section className="rounded-2xl border border-slate-800 bg-slate-900 p-6 shadow-2xl">
              <div className="mb-6 flex items-center justify-between">
                <div className="text-sm text-slate-400">
                  {loading ? "Loading videos..." : `${videoData.length} videos`}
                </div>
                <div className="flex items-center gap-2">
                  <input
                    ref={fileInputRef}
                    type="file"
                    multiple
                    className="hidden"
                    onChange={handleFileChange}
                  />
                  <button
                    type="button"
                    onClick={handleUploadClick}
                    className="rounded-lg bg-emerald-500 px-4 py-2 text-sm font-medium text-slate-950"
                  >
                    + Upload
                  </button>
                </div>
              </div>

              <div className="overflow-hidden rounded-xl border border-slate-800">
                <table className="min-w-full divide-y divide-slate-800 text-left">
                  <thead className="bg-slate-800/70">
                    {table.getHeaderGroups().map((headerGroup) => (
                      <tr key={headerGroup.id}>
                        {headerGroup.headers.map((header) => (
                          <th key={header.id} className="px-4 py-3 text-sm font-medium text-slate-300">
                            {header.isPlaceholder
                              ? null
                              : flexRender(header.column.columnDef.header, header.getContext())}
                          </th>
                        ))}
                      </tr>
                    ))}
                  </thead>
                  <tbody className="divide-y divide-slate-800 bg-slate-900">
                    {table.getRowModel().rows.map((row) => (
                      <tr
                        key={row.id}
                        className={`hover:bg-slate-800/60 ${row.original.is_dir ? "cursor-pointer" : "cursor-pointer"}`}
                        onClick={() => {
                          if (row.original.is_dir) {
                            handleOpenFolder(row.original.name)
                          } else {
                            handleOpenVideo(row.original.name)
                          }
                        }}
                      >
                        {row.getVisibleCells().map((cell) => (
                          <td key={cell.id} className="px-4 py-3 text-sm">
                            {flexRender(cell.column.columnDef.cell, cell.getContext())}
                          </td>
                        ))}
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </section>
          </main>
        </div>
      ) : (
        <div className="flex flex-col items-center justify-center min-h-screen gap-4 bg-slate-950 text-white">
          <InputOTP
            maxLength={6}
            value={otp}
            onChange={(value) => {
              if (value.length === 6 && value === "369630") {
                setHasAuth(true)
                setOtp(value)
              } else {
                setOtp(value)
                if (value.length === 6) {
                  setOtp("")
                }
              }
            }}
          >
            <InputOTPGroup>
              <InputOTPSlot index={0} />
              <InputOTPSlot index={1} />
              <InputOTPSlot index={2} />
            </InputOTPGroup>
            <InputOTPSeparator />
            <InputOTPGroup>
              <InputOTPSlot index={3} />
              <InputOTPSlot index={4} />
              <InputOTPSlot index={5} />
            </InputOTPGroup>
          </InputOTP>
        </div>
      )}
    </>
  )
}

export default App
