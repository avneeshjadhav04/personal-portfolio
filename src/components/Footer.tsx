export default function Footer() {
  return (
    <footer className="relative bg-text-primary border-t border-border overflow-hidden text-surface">
      <div className="max-w-7xl mx-auto px-6 md:px-10 py-20 md:py-32">
        <div className="grid md:grid-cols-4 gap-16 md:gap-12 mb-24">
          {/* Brand */}
          <div className="md:col-span-2">
            <h3 className="text-4xl md:text-5xl font-bold tracking-tighter uppercase mb-6">
              Avneesh <span className="text-gradient">Jadhav.</span>
            </h3>
            <p className="text-lg text-surface/70 leading-relaxed max-w-md font-light">
              AI Engineer building production systems at the intersection of
              machine learning, automation, and functional design.
            </p>
          </div>

          {/* Navigation */}
          <div>
            <h4 className="text-[12px] font-mono-accent uppercase tracking-[0.3em] text-surface/50 mb-8 block border-l-2 border-accent pl-4">
              Navigation
            </h4>
            <ul className="space-y-4">
              {['About', 'Projects', 'Skills', 'Experience', 'Certifications', 'Contact'].map((link) => (
                <li key={link}>
                  <a
                    href={`#${link.toLowerCase()}`}
                    className="text-base text-surface hover:text-accent transition-colors font-medium uppercase tracking-wide"
                  >
                    {link}
                  </a>
                </li>
              ))}
            </ul>
          </div>

          {/* Social */}
          <div>
            <h4 className="text-[12px] font-mono-accent uppercase tracking-[0.3em] text-surface/50 mb-8 block border-l-2 border-accent pl-4">
              Connect
            </h4>
            <ul className="space-y-4">
              <li>
                <a
                  href="https://www.linkedin.com/in/avneeshjadhav/"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-base text-surface hover:text-accent transition-colors font-medium uppercase tracking-wide"
                >
                  LinkedIn
                </a>
              </li>
              <li>
                <a
                  href="https://github.com/avneeshjadhav04"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-base text-surface hover:text-accent transition-colors font-medium uppercase tracking-wide"
                >
                  GitHub
                </a>
              </li>
              <li>
                <a
                  href="mailto:avneeshjadhav1@gmail.com"
                  className="text-base text-surface hover:text-accent transition-colors font-medium uppercase tracking-wide"
                >
                  Email
                </a>
              </li>
            </ul>
          </div>
        </div>

        {/* Bottom bar */}
        <div className="flex flex-col md:flex-row items-center justify-between gap-6 pt-10 border-t border-surface/20">
          <p className="text-xs text-surface/50 font-mono-accent uppercase tracking-widest">
            © {new Date().getFullYear()} Avneesh Jadhav. All rights reserved.
          </p>

          {/* Website Operational */}
          <div className="flex items-center gap-3 bg-surface/10 px-4 py-2">
            <span className="w-2 h-2 bg-green-500 rounded-full animate-pulse" />
            <span className="text-[10px] font-mono-accent uppercase tracking-widest text-surface/80">
              Website Operational
            </span>
          </div>
        </div>
      </div>
    </footer>
  );
}
