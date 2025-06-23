interface SectionTitleProps {
  title: string
  className?: string
}

export default function SectionTitle({ title, className = "" }: SectionTitleProps) {
  return (
    <div className={`flex flex-col items-center mb-16 animate-fade-in-up ${className}`}>
      <h2 className="text-4xl font-bold mb-4 text-center">{title}</h2>
      <div className="w-16 h-1 bg-purple-500 rounded"></div>
    </div>
  )
}
