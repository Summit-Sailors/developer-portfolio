import Image from "next/image"
import { PERSONAL_INFO, STATS } from "@/constants/data"
import SectionTitle from "@/components/ui/section-title"

export default function StoryPage() {
  return (
    <div className="py-24">
      <div className="container mx-auto px-4">
        <SectionTitle title="My Story" />
        <div className="max-w-3xl mx-auto">
          <div className="flex flex-col md:flex-row items-center gap-12 mb-16">
            <div className="md:w-1/3 animate-slide-in-left">
              <div className="relative size-48 md:size-64 mx-auto">
                <div className="absolute inset-0 rounded-full bg-gradient-to-r from-purple-500 to-cyan-500 blur-xl opacity-30 animate-pulse-slow"></div>
                <div className="relative w-full h-full rounded-full overflow-hidden border-4 border-zinc-800">
                  <Image
                    src={`/placeholder.svg?height=256&width=256&query=Jeremy+Meek+illustration`}
                    alt={PERSONAL_INFO.name}
                    width={256}
                    height={256}
                    className="object-cover"
                  />
                </div>
              </div>
            </div>
            <div className="md:w-2/3 animate-slide-in-right text-center md:text-left">
              <h1 className="text-4xl font-bold mb-4">{PERSONAL_INFO.name}</h1>
              <p className="text-xl text-purple-400 mb-6">{PERSONAL_INFO.title}</p>
              <p className="text-zinc-300 leading-relaxed">{PERSONAL_INFO.storyIntro}</p>
            </div>
          </div>

          <div className="space-y-6 text-zinc-300 leading-relaxed animate-fade-in-up animation-delay-300">
            {PERSONAL_INFO.storyDetail.map((paragraph, index) => (
              <p key={index}>{paragraph}</p>
            ))}
          </div>

          <div className="mt-16 animate-fade-in-up animation-delay-500">
            <h3 className="text-2xl font-bold mb-8 text-center">Key Milestones</h3>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-6">
              {STATS.map((stat, index) => (
                <div
                  key={index}
                  className="bg-zinc-900 p-6 rounded-lg border border-zinc-800 hover:border-purple-500 transition-colors"
                >
                  <h4 className="text-xl font-bold mb-2 text-purple-400">{stat.value}</h4>
                  <p className="text-zinc-300">{stat.label}</p>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
