import { PERSONAL_INFO } from "@/constants/data"
import SectionTitle from "@/components/ui/section-title"
import { FiMail, FiPhone, FiMapPin, FiGithub, FiLinkedin, FiTwitter } from "react-icons/fi"

export default function ContactPage() {
  return (
    <div className="py-24">
      <div className="container mx-auto px-4">
        <SectionTitle title="Get In Touch" />
        <div className="max-w-4xl mx-auto">
          <div className="bg-zinc-900 p-8 md:p-12 rounded-lg border border-zinc-800 shadow-xl animate-fade-in-up">
            <div className="grid md:grid-cols-2 gap-10 md:gap-16">
              <div className="space-y-8">
                <div>
                  <h3 className="text-2xl font-bold mb-4 text-white">Contact Information</h3>
                  <p className="text-zinc-400 mb-6">
                    Feel free to reach out via email, phone, or connect with me on social media. I'm always open to
                    discussing new projects, creative ideas, or opportunities to be part of something amazing.
                  </p>
                  <div className="space-y-5">
                    <div className="flex items-start space-x-4">
                      <div className="p-3 bg-zinc-800 rounded-full mt-1">
                        <FiMail size={20} className="text-purple-500" />
                      </div>
                      <div>
                        <p className="text-zinc-300 font-medium">Email</p>
                        <a
                          href={`mailto:${PERSONAL_INFO.email}`}
                          className="text-purple-400 hover:text-purple-300 transition-colors"
                        >
                          {PERSONAL_INFO.email}
                        </a>
                      </div>
                    </div>
                    <div className="flex items-start space-x-4">
                      <div className="p-3 bg-zinc-800 rounded-full mt-1">
                        <FiPhone size={20} className="text-purple-500" />
                      </div>
                      <div>
                        <p className="text-zinc-300 font-medium">Phone</p>
                        <a
                          href={`tel:${PERSONAL_INFO.phone.replace(/\s+/g, "")}`}
                          className="text-purple-400 hover:text-purple-300 transition-colors"
                        >
                          {PERSONAL_INFO.phone}
                        </a>
                      </div>
                    </div>
                    <div className="flex items-start space-x-4">
                      <div className="p-3 bg-zinc-800 rounded-full mt-1">
                        <FiMapPin size={20} className="text-purple-500" />
                      </div>
                      <div>
                        <p className="text-zinc-300 font-medium">Location</p>
                        <p className="text-zinc-400">{PERSONAL_INFO.location}</p>
                      </div>
                    </div>
                  </div>
                </div>
                <div>
                  <h3 className="text-xl font-bold mb-4 text-white">Follow Me</h3>
                  <div className="flex space-x-4">
                    <a
                      href={PERSONAL_INFO.github}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="p-3 bg-zinc-800 rounded-full text-zinc-400 hover:text-purple-500 hover:bg-zinc-700 transition-all"
                    >
                      <FiGithub size={22} />
                    </a>
                    <a
                      href={PERSONAL_INFO.linkedin}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="p-3 bg-zinc-800 rounded-full text-zinc-400 hover:text-purple-500 hover:bg-zinc-700 transition-all"
                    >
                      <FiLinkedin size={22} />
                    </a>
                    <a
                      href={PERSONAL_INFO.twitter}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="p-3 bg-zinc-800 rounded-full text-zinc-400 hover:text-purple-500 hover:bg-zinc-700 transition-all"
                    >
                      <FiTwitter size={22} />
                    </a>
                  </div>
                </div>
              </div>

              <div>
                <h3 className="text-2xl font-bold mb-6 text-white">Send a Message</h3>
                <form className="space-y-5">
                  <div>
                    <label htmlFor="name" className="block text-sm font-medium text-zinc-300 mb-1">
                      Your Name
                    </label>
                    <input
                      type="text"
                      id="name"
                      name="name"
                      placeholder="e.g. Jane Doe"
                      className="w-full px-4 py-3 bg-zinc-800 border border-zinc-700 rounded-lg focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-white"
                    />
                  </div>
                  <div>
                    <label htmlFor="email" className="block text-sm font-medium text-zinc-300 mb-1">
                      Your Email
                    </label>
                    <input
                      type="email"
                      id="email"
                      name="email"
                      placeholder="e.g. jane@example.com"
                      className="w-full px-4 py-3 bg-zinc-800 border border-zinc-700 rounded-lg focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-white"
                    />
                  </div>
                  <div>
                    <label htmlFor="subject" className="block text-sm font-medium text-zinc-300 mb-1">
                      Subject
                    </label>
                    <input
                      type="text"
                      id="subject"
                      name="subject"
                      placeholder="e.g. Project Inquiry"
                      className="w-full px-4 py-3 bg-zinc-800 border border-zinc-700 rounded-lg focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-white"
                    />
                  </div>
                  <div>
                    <label htmlFor="message" className="block text-sm font-medium text-zinc-300 mb-1">
                      Your Message
                    </label>
                    <textarea
                      id="message"
                      name="message"
                      placeholder="Your message here..."
                      rows={5}
                      className="w-full px-4 py-3 bg-zinc-800 border border-zinc-700 rounded-lg focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-white"
                    ></textarea>
                  </div>
                  <button
                    type="submit"
                    className="w-full px-6 py-3 bg-gradient-to-r from-purple-600 to-cyan-600 rounded-lg font-semibold text-white hover:opacity-90 transition-opacity focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2 focus:ring-offset-zinc-900"
                  >
                    Send Message
                  </button>
                </form>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
