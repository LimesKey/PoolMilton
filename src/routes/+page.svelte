<script lang="ts">
    let time = 'test';
        // When using the Tauri API npm package:
    import { invoke } from '@tauri-apps/api/tauri';

    async function get_stuff() {
        try {
        time = await invoke('display_time')
        console.log(time)
        }
        catch(error) {
            console.error('Error occurred:', error);
        }
    }
    get_stuff()

    let services = [
        { title: "Chemical Analysis", description: "Comprehensive chemical analysis of your pool water to ensure safety and clarity." },
        { title: "pH Testing", description: "Accurate pH testing to maintain the perfect balance in your pool." },
        { title: "Contaminant Detection", description: "Detect and eliminate harmful contaminants to keep your pool water clean." },
    ];
    let pricing = [
        { plan: "Twice a Week", price: "$120/month", description: "Ensure your pool is always in top condition with bi-weekly testing and maintenance." },
        { plan: "Once a Week", price: "$70/month", description: "Regular weekly testing to maintain the balance and cleanliness of your pool." },
        { plan: "Once a Month", price: "$30/month", description: "Basic monthly testing for low-maintenance pools." }
    ];
</script>

<style>
    :global(body) {
        margin: 0;
        font-family: 'Helvetica Neue', Arial, sans-serif;
        background-color: #121212;
        color: #f1f1f1;
    }
    header {
        background: #1e88e5;
        padding: 1rem 0;
        text-align: center;
        color: white;
    }
    nav a {
        margin: 0 15px;
        color: white;
        text-decoration: none;
        font-weight: bold;
    }
    .hero {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 80vh;
        background: linear-gradient(135deg, #1e88e5 0%, #42a5f5 100%);
        color: white;
        text-align: center;
    }
    .hero h1 {
        font-size: 3.5rem;
        margin-bottom: 0.5rem;
    }
    .hero p {
        font-size: 1.5rem;
        margin-bottom: 2rem;
    }
    .button {
        padding: 1rem 2rem;
        background: white;
        color: #1e88e5;
        border: none;
        border-radius: 25px;
        cursor: pointer;
        font-size: 1.2rem;
        transition: background 0.3s;
    }
    .button:hover {
        background: #f1f1f1;
    }
    .services, .pricing {
        padding: 4rem 2rem;
        background: #212121;
    }
    .services h2, .pricing h2 {
        text-align: center;
        margin-bottom: 3rem;
    }
    .service-item, .pricing-item {
        background: #333;
        padding: 2rem;
        border-radius: 10px;
        margin-bottom: 2rem;
        text-align: center;
        transition: transform 0.3s;
    }
    .service-item:hover, .pricing-item:hover {
        transform: translateY(-10px);
    }
    .service-item h3, .pricing-item h3 {
        margin-bottom: 1rem;
    }
</style>

<header>
    <nav>
        <a href="#">Home</a>
        <a href="#services">Services</a>
        <a href="#pricing">Pricing</a>
        <a href="#contact">Contact</a>
    </nav>
</header>

<section class="hero">
    <h1>Milton Pool Water Testing</h1>
    <p>Ensuring your pool water is safe, clean, and crystal clear.</p>
    <button class="button">Get Started</button>
</section>

<section id="services" class="services">
    <h2>Our Services</h2>
    {#each services as service}
        <div class="service-item">
            <h3>{service.title}</h3>
            <p>{service.description}</p>
        </div>
    {/each}
</section>

<section id="pricing" class="pricing">
    <h2>Pricing Options</h2>
    {#each pricing as plan}
        <div class="pricing-item">
            <h3>{plan.plan}</h3>
            <p>{plan.price}</p>
            <p>{plan.description}</p>
        </div>
    {/each}
</section>

<section id="contact" class="contact">
    <h2>Contact Us</h2>
    <p>Email: info@miltonpooltesting.com</p>
    <p>Phone: (123) 456-7890</p>
</section>
