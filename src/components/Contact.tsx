import { useState } from 'react';
import { motion } from 'framer-motion';
import { Send, MapPin, Phone, CheckCircle } from 'lucide-react';
import { LinkedInIcon, EmailIcon } from './Icons';

export default function Contact() {
  const [submitted, setSubmitted] = useState(false);
  const [formData, setFormData] = useState({
    name: '',
    email: '',
    message: '',
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const subject = encodeURIComponent(`Portfolio Contact from ${formData.name}`);
    const body = encodeURIComponent(
      `Name: ${formData.name}\nEmail: ${formData.email}\n\nMessage:\n${formData.message}`
    );
    window.location.href = `mailto:avneeshjadhav1@gmail.com?subject=${subject}&body=${body}`;
    setSubmitted(true);
    setTimeout(() => setSubmitted(false), 5000);
    setFormData({ name: '', email: '', message: '' });
  };

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

        <div className="grid md:grid-cols-2 gap-16">
          <motion.div
            initial={{ opacity: 0, x: -50 }}
            whileInView={{ opacity: 1, x: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.7, ease: "easeOut" }}
          >
            <h4 className="text-2xl font-bold mb-6">Let's build something together.</h4>
            <p className="text-text-secondary leading-relaxed mb-8">
              Whether you have a project in mind, need help with <span className="font-mono-accent text-accent-teal">AI/ML</span> solutions,
              or want to discuss the latest in tech — feel free to reach out.
              I'm based in Pune and open to remote opportunities worldwide.
            </p>

            <div className="space-y-6">
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
            </div>
          </motion.div>

          <motion.div
            initial={{ opacity: 0, x: 50 }}
            whileInView={{ opacity: 1, x: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.7, ease: "easeOut" }}
          >
            <form onSubmit={handleSubmit} className="space-y-6 p-8 rounded-2xl glass-card">
              <div>
                <label htmlFor="name" className="block text-sm font-medium mb-2 font-mono-accent text-accent-teal">
                  Name
                </label>
                <input
                  type="text"
                  id="name"
                  required
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  className="w-full px-4 py-3 rounded-xl bg-surface border border-border text-text-primary placeholder-text-secondary/50 focus:outline-none focus:border-accent-teal transition-colors"
                  placeholder="Your Name"
                />
              </div>

              <div>
                <label htmlFor="email" className="block text-sm font-medium mb-2 font-mono-accent text-accent-teal">
                  Email
                </label>
                <input
                  type="email"
                  id="email"
                  required
                  value={formData.email}
                  onChange={(e) => setFormData({ ...formData, email: e.target.value })}
                  className="w-full px-4 py-3 rounded-xl bg-surface border border-border text-text-primary placeholder-text-secondary/50 focus:outline-none focus:border-accent-teal transition-colors"
                  placeholder="you@example.com"
                />
              </div>

              <div>
                <label htmlFor="message" className="block text-sm font-medium mb-2 font-mono-accent text-accent-teal">
                  Message
                </label>
                <textarea
                  id="message"
                  required
                  rows={5}
                  value={formData.message}
                  onChange={(e) => setFormData({ ...formData, message: e.target.value })}
                  className="w-full px-4 py-3 rounded-xl bg-surface border border-border text-text-primary placeholder-text-secondary/50 focus:outline-none focus:border-accent-teal transition-colors resize-none"
                  placeholder="Tell me about your project or idea..."
                />
              </div>

              <button
                type="submit"
                disabled={submitted}
                className="w-full flex items-center justify-center gap-2 px-8 py-4 rounded-xl bg-gradient-to-r from-accent-teal to-accent-indigo text-background font-semibold hover:opacity-90 transition-opacity disabled:opacity-70 btn-glow"
              >
                {submitted ? (
                  <>
                    <CheckCircle size={20} />
                    Message Sent!
                  </>
                ) : (
                  <>
                    <Send size={20} />
                    Send Message
                  </>
                )}
              </button>
            </form>
          </motion.div>
        </div>
      </div>
    </section>
  );
}
