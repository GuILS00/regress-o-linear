use timewise_analytics::{linear_regression, calcular_r2, calcular_mse, prever_valores};

#[test]
fn test_linear_regression_valida() {
    let x = vec![1.0, 2.0, 3.0, 4.0];
    let y = vec![2.0, 4.0, 6.0, 8.0];
    let result = linear_regression(&x, &y).unwrap();
    let (a, b) = result;
    assert!((a - 2.0).abs() < 1e-6);
    assert!((b - 0.0).abs() < 1e-6);
}

#[test]
fn test_linear_regression_entrada_invalida() {
    let x = vec![1.0, 2.0];
    let y = vec![3.0]; // tamanho diferente
    let result = linear_regression(&x, &y);
    assert!(result.is_err());
}

#[test]
fn test_r2_perfeito() {
    let x = vec![0.0, 1.0, 2.0, 3.0];
    let y = vec![1.0, 3.0, 5.0, 7.0];
    let (a, b) = linear_regression(&x, &y).unwrap();
    let r2 = calcular_r2(&x, &y, a, b);
    assert!((r2 - 1.0).abs() < 1e-6);
}

#[test]
fn test_mse_zero() {
    let x = vec![0.0, 1.0, 2.0];
    let y = vec![2.0, 4.0, 6.0];
    let (a, b) = linear_regression(&x, &y).unwrap();
    let mse = calcular_mse(&x, &y, a, b);
    assert!((mse - 0.0).abs() < 1e-6);
}

#[test]
fn test_prever_valores() {
    let a = 2.0;
    let b = 1.0;
    let novos_x = vec![5.0, 6.0, 7.0];
    let esperados = vec![11.0, 13.0, 15.0];
    let previsoes = prever_valores(&novos_x, a, b);
    for (p, e) in previsoes.iter().zip(esperados.iter()) {
        assert!((*p - *e).abs() < 1e-6);
    }
}

#[test]
fn teste_previsao_linear() {
    use timewise_analytics::linear::{linear_regression, prever_valores, calcular_r2, calcular_mse};


    let y: Vec<f64> = vec![1.0, 2.1, 2.9, 4.2, 5.1];
    let x: Vec<f64> = (0..y.len()).map(|i| i as f64).collect();

    let (a, b) = linear_regression(&x, &y).expect("Falha na regressão");

    let previsoes = prever_valores(&[5.0, 6.0, 7.0], a, b);
    println!("Previsões para os próximos 3 pontos: {:?}", previsoes);

    let r2 = calcular_r2(&x, &y, a, b);
    let mse = calcular_mse(&x, &y, a, b);

    println!("R²: {:.4}", r2);
    println!("MSE: {:.4}", mse);

    assert!(r2 > 0.95);
    assert!(mse < 0.2);
}
