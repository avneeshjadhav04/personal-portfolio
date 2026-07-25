import { motion } from 'framer-motion';
import { MapPin, Phone } from 'lucide-react';
import { LinkedInIcon, EmailIcon, GitHubIcon } from './Icons';
import { viewportOnce, easeSmooth } from '../lib/motion';

export default function Contact() {
  return (
    <section id="contact" className="pt-16 pb-24 md:pt-24 md:pb-32 px-6 relative" style={{ contain: 'paint' }}>
      <div className="max-w-6xl mx-auto">
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={viewportOnce}
          transition={{ duration: 0.7, ease: easeSmooth }}
          className="text-center mb-16"
        >
          <h2 className="text-3xl md:text-5xl font-bold mb-16 tracking-tight text-center">Get in Touch</h2>
        </motion.div>

        <div className="max-w-2xl mx-auto">
          <motion.div
            initial={{ opacity: 0, y: 30 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={viewportOnce}
            transition={{ duration: 0.7, ease: easeSmooth }}
            className="space-y-6"
          >
            {[
              { icon: EmailIcon, label: 'Email', value: 'avneeshjadhav1@gmail.com', href: 'mailto:avneeshjadhav1@gmail.com' },
              { icon: Phone, label: 'Phone', value: '+91 95454 57385', href: 'tel:+919545457385' },
              { icon: MapPin, label: 'Location', value: 'Pune, Maharashtra, India', href: null },
              { icon: LinkedInIcon, label: 'LinkedIn', value: 'linkedin.com/in/avneeshjadhav', href: 'https://www.linkedin.com/in/avneeshjadhav/' },
              { icon: GitHubIcon, label: 'GitHub', value: 'github.com/avneeshjadhav04', href: 'https://github.com/avneeshjadhav04' },
            ].map(({ icon: Icon, label, value, href }) => (
              <div key={label} className="flex items-center gap-4 p-4 rounded-xl glass-card">
                <div className="p-3 rounded-xl bg-accent-teal/10 border border-accent-teal/20">
                  <Icon size={20} className="text-accent-teal" />
                </div>
                <div>
                  <p className="text-sm text-text-secondary font-mono-accent">{label}</p>
                  {href ? (
                    <a 
                      href={href} 
                      target={href.startsWith('http') ? '_blank' : undefined}
                      rel={href.startsWith('http') ? 'noopener noreferrer' : undefined}
                      className="font-medium hover:text-accent-teal transition-colors link-underline"
                    >
                      {value}
                    </a>
                  ) : (
                    <p className="font-medium">{value}</p>
                  )}
                </div>
              </div>
            ))}
          </motion.div>
        </div>
      </div>
    </section>
  );
}
