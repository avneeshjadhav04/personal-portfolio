import { motion } from 'framer-motion';
import { MapPin, Phone } from 'lucide-react';
import { LinkedInIcon, EmailIcon } from './Icons';

export default function Contact() {
  return (
    <section id="contact" className="pt-32 pb-24 md:pt-40 md:pb-32 px-6 relative">
      <div className="max-w-6xl mx-auto">
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.7 }}
          className="text-center mb-16"
        >
          <h2 className="text-sm font-semibold text-accent-teal uppercase tracking-[0.2em] mb-4 font-mono-accent">
            Connect
          </h2>
          <h3 className="text-3xl md:text-5xl font-bold mb-6 tracking-tight">Get in Touch</h3>
          <p className="text-text-secondary max-w-2xl mx-auto leading-relaxed">
            Interested in collaborating on AI projects, automation solutions, or just want to connect?
            I'm always open to new opportunities.
          </p>
        </motion.div>

        <div className="max-w-2xl mx-auto">
          <motion.div
            initial={{ opacity: 0, y: 30 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.7, ease: "easeOut" }}
            className="space-y-6"
          >
            {[
              { icon: EmailIcon, label: 'Email', value: 'avneeshjadhav1@gmail.com', href: 'mailto:avneeshjadhav1@gmail.com' },
              { icon: Phone, label: 'Phone', value: '+91 95454 57385', href: 'tel:+919545457385' },
              { icon: MapPin, label: 'Location', value: 'Pune, Maharashtra, India', href: null },
              { icon: LinkedInIcon, label: 'LinkedIn', value: 'linkedin.com/in/avneeshjadhav', href: 'https://www.linkedin.com/in/avneeshjadhav/' },
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
